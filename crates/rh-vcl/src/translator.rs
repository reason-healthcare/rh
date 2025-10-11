//! VCL to FHIR ValueSet translator
//!
//! This module provides functionality to translate VCL expressions into FHIR ValueSet.compose structures.

use crate::{
    ast::{self, *},
    error::{VclError, VclResult},
    fhir::{self, *},
};

/// Translator for converting VCL expressions to FHIR ValueSet compose structures
#[derive(Debug, Clone)]
pub struct VclTranslator {
    /// Default system URI to use when none is specified
    pub default_system: Option<String>,

    /// Default version to use when none is specified and default system is used
    pub default_version: Option<String>,

    /// Whether to create separate includes for conjunctions (true) or combine them (false)
    pub separate_conjunction_includes: bool,
}

impl VclTranslator {
    /// Create a new translator with default settings
    pub fn new() -> Self {
        Self {
            default_system: None,
            default_version: None,
            separate_conjunction_includes: false, // Default to combining conjunctions
        }
    }

    /// Create a new translator with a default code system
    pub fn with_default_system(system: String) -> Self {
        // Parse system|version if present
        let (system, version) = if let Some(pipe_pos) = system.find('|') {
            let (sys, ver) = system.split_at(pipe_pos);
            (sys.to_string(), Some(ver[1..].to_string())) // Skip the '|' character
        } else {
            (system, None)
        };

        Self {
            default_system: Some(system),
            default_version: version,
            separate_conjunction_includes: false, // Default to combining conjunctions
        }
    }

    /// Create a new translator with a default code system and version
    pub fn with_default_system_and_version(system: String, version: Option<String>) -> Self {
        Self {
            default_system: Some(system),
            default_version: version,
            separate_conjunction_includes: false, // Default to combining conjunctions
        }
    }

    /// Set whether to create separate includes for conjunctions
    pub fn set_separate_conjunction_includes(mut self, separate: bool) -> Self {
        self.separate_conjunction_includes = separate;
        self
    }

    /// Translate a VCL expression into a FHIR ValueSet compose structure
    pub fn translate(&self, vcl_expr: &VclExpression) -> VclResult<ValueSetCompose> {
        // First, validate that the expression can be translated to FHIR compose
        self.is_expression_translatable(&vcl_expr.expr)?;

        let mut compose = ValueSetCompose::new();
        self.translate_expression(&vcl_expr.expr, &mut compose)?;
        Ok(compose)
    }

    /// Translate an expression into the compose structure
    fn translate_expression(
        &self,
        expr: &Expression,
        compose: &mut ValueSetCompose,
    ) -> VclResult<()> {
        // First translate the main sub-expression
        let main_include = self.translate_sub_expression(&expr.sub_expr)?;

        match &expr.operation {
            None => {
                // Simple expression - just add the include
                if let Some(include) = main_include {
                    compose.add_include(include);
                }
            }
            Some(Operation::Conjunction(sub_exprs)) => {
                // Conjunction: behavior depends on separate_conjunction_includes flag
                if self.separate_conjunction_includes {
                    // Create separate includes for each expression
                    if let Some(include) = main_include {
                        compose.add_include(include);
                    }

                    for sub_expr in sub_exprs {
                        if let Some(include) = self.translate_sub_expression(sub_expr)? {
                            compose.add_include(include);
                        }
                    }
                } else {
                    // Combine all expressions into concepts of a single include (like disjunction)
                    let (system, version) =
                        self.get_system_and_version_from_sub_expr(&expr.sub_expr);
                    let default_system =
                        system.unwrap_or_else(|| self.default_system.clone().unwrap_or_default());
                    let default_version = version.or_else(|| self.default_version.clone());
                    let mut combined_include =
                        ValueSetInclude::new_system_with_version(default_system, default_version);

                    // Add main expression concepts
                    if let Some(main_include) = main_include {
                        self.merge_include_into(&main_include, &mut combined_include)?;
                    }

                    // Add all conjunction concepts
                    for sub_expr in sub_exprs {
                        if let Some(include) = self.translate_sub_expression(sub_expr)? {
                            self.merge_include_into(&include, &mut combined_include)?;
                        }
                    }

                    compose.add_include(combined_include);
                }
            }
            Some(Operation::Disjunction(sub_exprs)) => {
                // Disjunction: Check if we should combine (concepts) or separate (filters)

                // First, translate all sub-expressions
                let sub_includes: Vec<ValueSetInclude> = sub_exprs
                    .iter()
                    .filter_map(|sub_expr| self.translate_sub_expression(sub_expr).ok().flatten())
                    .collect();

                if self.should_combine_disjunction_includes(&main_include, &sub_includes) {
                    // Combine all expressions into concepts of a single include (concept-only case)
                    let (system, version) =
                        self.get_system_and_version_from_sub_expr(&expr.sub_expr);
                    let default_system =
                        system.unwrap_or_else(|| self.default_system.clone().unwrap_or_default());
                    let default_version = version.or_else(|| self.default_version.clone());
                    let mut combined_include =
                        ValueSetInclude::new_system_with_version(default_system, default_version);

                    // Add main expression concepts
                    if let Some(main_include) = main_include {
                        self.merge_include_into(&main_include, &mut combined_include)?;
                    }

                    // Add all disjunction concepts
                    for include in sub_includes {
                        self.merge_include_into(&include, &mut combined_include)?;
                    }

                    if !combined_include.concept.is_empty() || !combined_include.filter.is_empty() {
                        compose.add_include(combined_include);
                    }
                } else {
                    // Create separate includes for filter-based expressions (true OR logic)
                    if let Some(include) = main_include {
                        compose.add_include(include);
                    }

                    for include in sub_includes {
                        compose.add_include(include);
                    }
                }
            }
            Some(Operation::Exclusion(excluded_expr)) => {
                // Exclusion: Main expression as include, excluded as exclude
                if let Some(include) = main_include {
                    compose.add_include(include);
                }

                if let Some(exclude) = self.translate_sub_expression(excluded_expr)? {
                    compose.add_exclude(exclude);
                }
            }
        }

        Ok(())
    }

    /// Translate a sub-expression into a ValueSet include
    fn translate_sub_expression(
        &self,
        sub_expr: &SubExpression,
    ) -> VclResult<Option<ValueSetInclude>> {
        let (system, version) = self.get_system_and_version_from_sub_expr(sub_expr);

        match &sub_expr.content {
            SubExpressionContent::Simple(simple_expr) => {
                self.translate_simple_expression(simple_expr, system, version)
            }
            SubExpressionContent::Nested(nested_expr) => {
                // For nested expressions, create a new compose and merge results
                let mut nested_compose = ValueSetCompose::new();

                // If the parent sub-expression has a system URI, create a temporary translator
                // with that system as the default for the nested expression
                if let (Some(inherited_system), inherited_version) = (system, version) {
                    let nested_translator = VclTranslator::with_default_system_and_version(
                        inherited_system,
                        inherited_version,
                    );
                    nested_translator.translate_expression(nested_expr, &mut nested_compose)?;
                } else {
                    self.translate_expression(nested_expr, &mut nested_compose)?;
                }

                // If all includes have the same system, merge their filters into one include
                if nested_compose.include.len() > 1 {
                    let first_system = nested_compose.include[0].system.clone();
                    let first_version = nested_compose.include[0].version.clone();
                    let all_same_system = nested_compose
                        .include
                        .iter()
                        .all(|inc| inc.system == first_system && inc.version == first_version);

                    if all_same_system
                        && nested_compose
                            .include
                            .iter()
                            .all(|inc| !inc.filter.is_empty())
                    {
                        // Merge all filters into a single include
                        let mut merged_include = ValueSetInclude::new_system_with_version(
                            first_system.unwrap_or_default(),
                            first_version,
                        );

                        for include in nested_compose.include {
                            for filter in include.filter {
                                merged_include.add_filter(filter.property, filter.op, filter.value);
                            }
                            // Also merge concepts if any
                            for concept in include.concept {
                                merged_include.add_concept(concept.code, concept.display);
                            }
                        }

                        Ok(Some(merged_include))
                    } else {
                        // Different systems or mixed content - take first include
                        if let Some(first_include) = nested_compose.include.into_iter().next() {
                            Ok(Some(first_include))
                        } else {
                            Ok(None)
                        }
                    }
                } else {
                    // Single or no includes - return as is
                    if let Some(first_include) = nested_compose.include.into_iter().next() {
                        Ok(Some(first_include))
                    } else {
                        Ok(None)
                    }
                }
            }
        }
    }

    /// Translate a simple expression into a ValueSet include
    fn translate_simple_expression(
        &self,
        simple_expr: &SimpleExpression,
        system: Option<String>,
        version: Option<String>,
    ) -> VclResult<Option<ValueSetInclude>> {
        let use_default_system = system.is_none();
        let system_uri = system.or_else(|| self.default_system.clone());
        let version_to_use = version.or_else(|| {
            // Only use default version if we're using the default system
            if use_default_system && self.default_system.is_some() {
                self.default_version.clone()
            } else {
                None
            }
        });

        match simple_expr {
            SimpleExpression::Wildcard => {
                // Wildcard - include all from system
                if let Some(sys) = system_uri {
                    let include = ValueSetInclude::new_system_with_version(sys, version_to_use);
                    // Add a filter that matches everything (no specific filter needed for wildcard)
                    Ok(Some(include))
                } else {
                    Err(VclError::parse_error(
                        "Wildcard requires a system URI",
                        0,
                        "",
                    ))
                }
            }
            SimpleExpression::Code(code) => {
                // Simple code
                if let Some(sys) = system_uri {
                    let mut include =
                        ValueSetInclude::new_system_with_version(sys, version_to_use.clone());
                    include.add_concept(code.value().to_string(), None);
                    Ok(Some(include))
                } else {
                    Err(VclError::parse_error("Code requires a system URI", 0, ""))
                }
            }
            SimpleExpression::Filter(filter) => {
                // Filter expression
                self.translate_filter(filter, system_uri, version_to_use)
            }
            SimpleExpression::FilterList(filters) => {
                // Filter list - translate as multiple filters on the same system
                self.translate_filter_list(filters, system_uri, version_to_use)
            }
            SimpleExpression::IncludeValueSet(include_vs) => {
                // Include ValueSet
                match include_vs {
                    IncludeValueSet::Uri(uri) => {
                        Ok(Some(ValueSetInclude::new_valueset(uri.clone())))
                    }
                    IncludeValueSet::SystemUri(system_uri) => {
                        Ok(Some(ValueSetInclude::new_system_with_version(
                            system_uri.uri.clone(),
                            system_uri.version.clone(),
                        )))
                    }
                }
            }
        }
    }

    /// Translate a filter into a ValueSet include
    fn translate_filter(
        &self,
        filter: &Filter,
        system: Option<String>,
        version: Option<String>,
    ) -> VclResult<Option<ValueSetInclude>> {
        let use_default_system = system.is_none();
        let system_uri = system
            .or_else(|| self.default_system.clone())
            .ok_or_else(|| VclError::parse_error("Filter requires a system URI", 0, ""))?;

        let version_to_use = version.or_else(|| {
            // Only use default version if we're using the default system
            if use_default_system && self.default_system.is_some() {
                self.default_version.clone()
            } else {
                None
            }
        });

        let mut include = ValueSetInclude::new_system_with_version(system_uri, version_to_use);

        match filter {
            Filter::PropertyFilter {
                property,
                operator,
                value,
            } => {
                // Check if this is a complex nested filter structure
                // Look for patterns like prop1^{prop2^{...}}
                if matches!(operator, ast::FilterOperator::In) {
                    if let FilterValue::FilterList(filters) = value {
                        if filters.len() == 1 {
                            if let Filter::PropertyFilter {
                                operator: inner_op,
                                value: inner_value,
                                ..
                            } = &filters[0]
                            {
                                // Check if inner filter is also using In operator with FilterList
                                if matches!(inner_op, ast::FilterOperator::In) {
                                    if let FilterValue::FilterList(_) = inner_value {
                                        return Err(VclError::parse_error(
                                            "Complex nested filters are not yet implemented",
                                            0,
                                            "Deeply nested filter structures like 'prop1^{prop2^{prop3=value}}' are not currently supported"
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }

                // Fallback to original single filter transformation logic
                let (final_op, final_value) = if matches!(operator, ast::FilterOperator::In) {
                    if let FilterValue::FilterList(filters) = value {
                        if filters.len() == 1 {
                            if let Filter::PropertyFilter {
                                property: _,
                                operator: inner_op,
                                value: inner_value,
                            } = &filters[0]
                            {
                                // Transform the operator: use the inner operator instead of In
                                let transformed_op = self.map_filter_operator(inner_op)?;
                                let transformed_value = self.map_filter_value(inner_value)?;
                                (transformed_op, transformed_value)
                            } else {
                                // Other filter types, fall back to original behavior
                                let fhir_op = self.map_filter_operator(operator)?;
                                let filter_value = self.map_filter_value(value)?;
                                (fhir_op, filter_value)
                            }
                        } else {
                            // Multiple filters, fall back to original behavior
                            let fhir_op = self.map_filter_operator(operator)?;
                            let filter_value = self.map_filter_value(value)?;
                            (fhir_op, filter_value)
                        }
                    } else {
                        // Not a FilterList, normal processing
                        let fhir_op = self.map_filter_operator(operator)?;
                        let filter_value = self.map_filter_value(value)?;
                        (fhir_op, filter_value)
                    }
                } else {
                    // Not an In operator, normal processing
                    let fhir_op = self.map_filter_operator(operator)?;
                    let filter_value = self.map_filter_value(value)?;
                    (fhir_op, filter_value)
                };

                include.add_filter(property.value().to_string(), final_op, final_value);
            }
            Filter::OfOperation { source, property } => {
                // "Of" operations - choose operator based on source type
                let fhir_op = match source {
                    OfSource::CodeList(_) => fhir::FilterOperator::In, // Use "in" for code lists
                    _ => fhir::FilterOperator::Equal, // Default to equality for single codes, URIs, etc.
                };
                let source_value = self.map_of_source_to_value(source)?;
                include.add_filter(property.value().to_string(), fhir_op, source_value);
            }
        }

        Ok(Some(include))
    }

    /// Translate a list of filters into a ValueSet include
    fn translate_filter_list(
        &self,
        filters: &[Filter],
        system: Option<String>,
        version: Option<String>,
    ) -> VclResult<Option<ValueSetInclude>> {
        let use_default_system = system.is_none();
        let system_uri = system
            .or_else(|| self.default_system.clone())
            .ok_or_else(|| VclError::parse_error("Filter list requires a system URI", 0, ""))?;

        let version_to_use = version.or_else(|| {
            // Only use default version if we're using the default system
            if use_default_system && self.default_system.is_some() {
                self.default_version.clone()
            } else {
                None
            }
        });

        let mut include = ValueSetInclude::new_system_with_version(system_uri, version_to_use);

        // Add all filters to the same include
        for filter in filters {
            match filter {
                Filter::PropertyFilter {
                    property,
                    operator,
                    value,
                } => {
                    // Special handling for In operator with FilterList (same as in translate_filter)
                    let (final_op, final_value) = if matches!(operator, ast::FilterOperator::In) {
                        if let FilterValue::FilterList(filters) = value {
                            if filters.len() == 1 {
                                if let Filter::PropertyFilter {
                                    property: _,
                                    operator: inner_op,
                                    value: inner_value,
                                } = &filters[0]
                                {
                                    let transformed_op = self.map_filter_operator(inner_op)?;
                                    let transformed_value = self.map_filter_value(inner_value)?;
                                    (transformed_op, transformed_value)
                                } else {
                                    let fhir_op = self.map_filter_operator(operator)?;
                                    let filter_value = self.map_filter_value(value)?;
                                    (fhir_op, filter_value)
                                }
                            } else {
                                let fhir_op = self.map_filter_operator(operator)?;
                                let filter_value = self.map_filter_value(value)?;
                                (fhir_op, filter_value)
                            }
                        } else {
                            let fhir_op = self.map_filter_operator(operator)?;
                            let filter_value = self.map_filter_value(value)?;
                            (fhir_op, filter_value)
                        }
                    } else {
                        let fhir_op = self.map_filter_operator(operator)?;
                        let filter_value = self.map_filter_value(value)?;
                        (fhir_op, filter_value)
                    };

                    include.add_filter(property.value().to_string(), final_op, final_value);
                }
                Filter::OfOperation { source, property } => {
                    // "Of" operations in filter lists - for now, create a filter based on the property
                    let fhir_op = match source {
                        OfSource::CodeList(_) => fhir::FilterOperator::In,
                        _ => fhir::FilterOperator::Equal,
                    };
                    let source_value = self.map_of_source_to_value(source)?;
                    include.add_filter(property.value().to_string(), fhir_op, source_value);
                }
            }
        }

        Ok(Some(include))
    }

    /// Convert a filter to a string representation (for nested filter lists)
    fn filter_to_string(&self, filter: &Filter) -> VclResult<String> {
        match filter {
            Filter::PropertyFilter {
                property,
                operator,
                value,
            } => {
                let op_str = match operator {
                    ast::FilterOperator::Equals => "=",
                    ast::FilterOperator::IsA => "<<",
                    ast::FilterOperator::IsNotA => "~<<",
                    ast::FilterOperator::DescendantOf => "<",
                    ast::FilterOperator::Regex => "/",
                    ast::FilterOperator::In => "^",
                    ast::FilterOperator::NotIn => "~^",
                    ast::FilterOperator::Generalizes => ">>",
                    ast::FilterOperator::ChildOf => "<!",
                    ast::FilterOperator::DescendantLeaf => "!!<",
                    ast::FilterOperator::Exists => "?",
                };
                let value_str = match value {
                    FilterValue::Code(code) => code.value().to_string(),
                    FilterValue::String(s) => format!("\"{s}\""),
                    FilterValue::CodeList(codes) => {
                        let code_strs: Vec<String> =
                            codes.iter().map(|c| c.value().to_string()).collect();
                        let joined = code_strs.join(",");
                        format!("{{{joined}}}")
                    }
                    FilterValue::Uri(uri) => uri.clone(),
                    FilterValue::FilterList(_) => {
                        // For deeply nested filter lists, just use a placeholder
                        "{...}".to_string()
                    }
                };
                let prop_value = property.value();
                Ok(format!("{prop_value}{op_str}:{value_str}"))
            }
            Filter::OfOperation { source, property } => {
                let source_str = match source {
                    OfSource::Code(code) => code.value().to_string(),
                    OfSource::CodeList(codes) => {
                        let code_strs: Vec<String> =
                            codes.iter().map(|c| c.value().to_string()).collect();
                        let joined = code_strs.join(",");
                        format!("{{{joined}}}")
                    }
                    OfSource::Wildcard => "*".to_string(),
                    OfSource::Uri(uri) => uri.clone(),
                    OfSource::FilterList(_) => "{...}".to_string(),
                };
                let prop_value = property.value();
                Ok(format!("{source_str}.{prop_value}"))
            }
        }
    }

    /// Map VCL filter operator to FHIR filter operator
    fn map_filter_operator(&self, vcl_op: &ast::FilterOperator) -> VclResult<fhir::FilterOperator> {
        let fhir_op = match vcl_op {
            ast::FilterOperator::Equals => fhir::FilterOperator::Equal,
            ast::FilterOperator::IsA => fhir::FilterOperator::IsA,
            ast::FilterOperator::IsNotA => fhir::FilterOperator::IsNotA,
            ast::FilterOperator::DescendantOf => fhir::FilterOperator::DescendentOf,
            ast::FilterOperator::Regex => fhir::FilterOperator::Regex,
            ast::FilterOperator::In => fhir::FilterOperator::In,
            ast::FilterOperator::NotIn => fhir::FilterOperator::NotIn,
            ast::FilterOperator::Generalizes => fhir::FilterOperator::Generalizes,
            ast::FilterOperator::ChildOf => fhir::FilterOperator::ChildOf,
            ast::FilterOperator::DescendantLeaf => fhir::FilterOperator::DescendentLeaf,
            ast::FilterOperator::Exists => fhir::FilterOperator::Exists,
        };
        Ok(fhir_op)
    }

    /// Map VCL filter value to string
    fn map_filter_value(&self, value: &FilterValue) -> VclResult<String> {
        match value {
            FilterValue::Code(code) => Ok(code.value().to_string()),
            FilterValue::String(s) => Ok(s.clone()),
            FilterValue::CodeList(codes) => {
                // Join codes with comma for list values
                let code_strs: Vec<String> = codes.iter().map(|c| c.value().to_string()).collect();
                Ok(code_strs.join(","))
            }
            FilterValue::Uri(uri) => Ok(uri.clone()),
            FilterValue::FilterList(filters) => {
                // Handle special case: single filter like {concept<<1151133}
                // This represents a set defined by the filter, so we extract the value
                if filters.len() == 1 {
                    if let Filter::PropertyFilter {
                        property: _,
                        operator: _,
                        value,
                    } = &filters[0]
                    {
                        // For {concept<<value}, we want to use the value directly
                        // The operator transformation is handled separately in the calling context
                        return self.map_filter_value(value);
                    }
                }

                // For multiple filters or complex cases, convert to string representation
                let filter_strs: Result<Vec<String>, _> =
                    filters.iter().map(|f| self.filter_to_string(f)).collect();
                match filter_strs {
                    Ok(strs) => {
                        let joined = strs.join(" AND ");
                        Ok(format!("({joined})"))
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }

    /// Map "of" source to filter value
    fn map_of_source_to_value(&self, source: &OfSource) -> VclResult<String> {
        match source {
            OfSource::Code(code) => Ok(code.value().to_string()),
            OfSource::CodeList(codes) => {
                let code_strs: Vec<String> = codes.iter().map(|c| c.value().to_string()).collect();
                Ok(code_strs.join(","))
            }
            OfSource::Wildcard => Ok("*".to_string()),
            OfSource::Uri(uri) => Ok(uri.clone()),
            OfSource::FilterList(_) => Err(VclError::parse_error(
                "Complex filter lists not supported in FHIR translation",
                0,
                "",
            )),
        }
    }

    /// Get system URI from sub-expression
    /// Get system URI and version from a sub-expression
    fn get_system_and_version_from_sub_expr(
        &self,
        sub_expr: &SubExpression,
    ) -> (Option<String>, Option<String>) {
        match &sub_expr.system_uri {
            Some(uri) => (Some(uri.uri.clone()), uri.version.clone()),
            _ => (None, None),
        }
    }

    /// Merge one include into another
    fn merge_include_into(
        &self,
        source: &ValueSetInclude,
        target: &mut ValueSetInclude,
    ) -> VclResult<()> {
        // Merge concepts
        for concept in &source.concept {
            if !target.concept.iter().any(|c| c.code == concept.code) {
                target.concept.push(concept.clone());
            }
        }

        // Merge filters
        for filter in &source.filter {
            if !target.filter.iter().any(|f| {
                f.property == filter.property && f.op == filter.op && f.value == filter.value
            }) {
                target.filter.push(filter.clone());
            }
        }

        // Merge valueset references
        for vs in &source.value_set {
            if !target.value_set.contains(vs) {
                target.value_set.push(vs.clone());
            }
        }

        Ok(())
    }

    /// Check if includes should be combined for disjunction (concepts) or kept separate (filters)
    fn should_combine_disjunction_includes(
        &self,
        main_include: &Option<ValueSetInclude>,
        sub_includes: &[ValueSetInclude],
    ) -> bool {
        // If main include has filters, don't combine
        if let Some(main) = main_include {
            if !main.filter.is_empty() {
                return false;
            }
        }

        // If any sub-include has filters, don't combine
        for include in sub_includes {
            if !include.filter.is_empty() {
                return false;
            }
        }

        // Only combine if all includes are concept-only (no filters)
        true
    }

    /// Check if an expression can be translated to FHIR ValueSet.compose
    /// Only filter expressions and codes are translatable, not value set expressions
    fn is_expression_translatable(&self, expr: &Expression) -> VclResult<()> {
        self.check_sub_expression_translatable(&expr.sub_expr)?;

        if let Some(ref operation) = expr.operation {
            match operation {
                Operation::Conjunction(sub_exprs) | Operation::Disjunction(sub_exprs) => {
                    for sub_expr in sub_exprs {
                        self.check_sub_expression_translatable(sub_expr)?;
                    }
                }
                Operation::Exclusion(sub_expr) => {
                    self.check_sub_expression_translatable(sub_expr)?;
                }
            }
        }

        Ok(())
    }

    /// Check if a sub-expression is translatable
    fn check_sub_expression_translatable(&self, sub_expr: &SubExpression) -> VclResult<()> {
        match &sub_expr.content {
            SubExpressionContent::Simple(simple) => {
                self.check_simple_expression_translatable(simple)?;
            }
            SubExpressionContent::Nested(nested_expr) => {
                self.is_expression_translatable(nested_expr)?;
            }
        }
        Ok(())
    }

    /// Check if a simple expression is translatable
    fn check_simple_expression_translatable(&self, simple: &SimpleExpression) -> VclResult<()> {
        use crate::ast::{Filter, SimpleExpression};

        match simple {
            SimpleExpression::Wildcard => Ok(()), // * is translatable as a concept
            SimpleExpression::Code(_) => Ok(()),  // Codes are translatable
            SimpleExpression::IncludeValueSet(_) => Ok(()), // ValueSet includes are translatable
            SimpleExpression::Filter(filter) => {
                // Check if this is a translatable filter
                match filter {
                    Filter::PropertyFilter { .. } => Ok(()), // Property filters are translatable
                    Filter::OfOperation { .. } => {
                        // Of operations represent value sets, not filters
                        Err(VclError::TranslationError {
                            message: "Of operations (like '*.concept' or 'B.codeprop') represent value sets and cannot be translated to FHIR ValueSet.compose filters. Only filter expressions with operators (=, <<, etc.) can be translated.".to_string()
                        })
                    }
                }
            }
            SimpleExpression::FilterList(filters) => {
                // Check each filter in the list
                for filter in filters {
                    match filter {
                        Filter::PropertyFilter { .. } => {} // OK
                        Filter::OfOperation { .. } => {
                            return Err(VclError::TranslationError {
                                message: "Filter lists containing of operations cannot be translated to FHIR ValueSet.compose. Only property filters with operators are supported.".to_string()
                            })
                        }
                    }
                }
                Ok(())
            }
        }
    }
}

impl Default for VclTranslator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to translate VCL expression to FHIR ValueSet compose
pub fn translate_vcl_to_fhir(vcl_expr: &VclExpression) -> VclResult<ValueSetCompose> {
    let translator = VclTranslator::new();
    translator.translate(vcl_expr)
}

/// Convenience function to translate VCL string to FHIR ValueSet compose  
pub fn translate_vcl_string_to_fhir(vcl_str: &str) -> VclResult<ValueSetCompose> {
    let expr = crate::parse_vcl(vcl_str)?;
    translate_vcl_to_fhir(&expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_vcl;

    #[test]
    fn test_simple_code_translation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("123456").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        assert_eq!(compose.exclude.len(), 0);

        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "123456");
    }

    #[test]
    fn test_code_with_system_uri() {
        let translator = VclTranslator::new();
        let expr = parse_vcl("(http://loinc.org)8302-2").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://loinc.org".to_string()));
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "8302-2");
    }

    #[test]
    fn test_code_with_system_uri_and_version() {
        let translator = VclTranslator::new();
        let expr = parse_vcl("(http://snomed.info/sct|2025)123456789").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.version, Some("2025".to_string()));
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "123456789");
    }

    #[test]
    fn test_code_with_system_uri_and_complex_version() {
        let translator = VclTranslator::new();
        let expr = parse_vcl("(http://loinc.org|v2.76)8302-2").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://loinc.org".to_string()));
        assert_eq!(include.version, Some("v2.76".to_string()));
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "8302-2");
    }

    #[test]
    fn test_default_system_with_version() {
        let translator =
            VclTranslator::with_default_system("http://snomed.info/sct|2025".to_string());
        let expr = parse_vcl("123456").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.version, Some("2025".to_string()));
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "123456");
    }

    #[test]
    fn test_default_system_with_wildcard_and_version() {
        let translator =
            VclTranslator::with_default_system("http://snomed.info/sct|2025".to_string());
        let expr = parse_vcl("*").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.version, Some("2025".to_string()));
        // Wildcard should not have concepts
        assert_eq!(include.concept.len(), 0);
    }

    #[test]
    fn test_quoted_code() {
        let translator = VclTranslator::with_default_system("http://example.org".to_string());
        let expr = parse_vcl("\"code with spaces\"").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "code with spaces");
    }

    #[test]
    fn test_wildcard_translation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("*").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        // Wildcard creates an include with no specific concepts or filters (includes all)
        assert_eq!(include.concept.len(), 0);
    }

    #[test]
    fn test_property_filter_equals() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("status = \"active\"").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 1);

        let filter = &include.filter[0];
        assert_eq!(filter.property, "status");
        assert_eq!(filter.op, fhir::FilterOperator::Equal);
        assert_eq!(filter.value, "active");
    }

    #[test]
    fn test_property_filter_is_a() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("category << 123456").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 1);

        let filter = &include.filter[0];
        assert_eq!(filter.property, "category");
        assert_eq!(filter.op, fhir::FilterOperator::IsA);
        assert_eq!(filter.value, "123456");
    }

    #[test]
    fn test_conjunction_translation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("code1, code2, code3").unwrap();
        let compose = translator.translate(&expr).unwrap();

        // Conjunction should create one include with multiple concepts (default behavior)
        assert_eq!(compose.include.len(), 1);
        assert_eq!(compose.exclude.len(), 0);

        let include = &compose.include[0];
        assert_eq!(include.concept.len(), 3);

        let codes: Vec<&str> = include.concept.iter().map(|c| c.code.as_str()).collect();
        assert!(codes.contains(&"code1"));
        assert!(codes.contains(&"code2"));
        assert!(codes.contains(&"code3"));
    }

    #[test]
    fn test_conjunction_translation_separate_includes() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string())
            .set_separate_conjunction_includes(true);
        let expr = parse_vcl("code1, code2, code3").unwrap();
        let compose = translator.translate(&expr).unwrap();

        // With separate_conjunction_includes=true, should create multiple includes
        assert_eq!(compose.include.len(), 3);
        assert_eq!(compose.exclude.len(), 0);

        let codes: Vec<&str> = compose
            .include
            .iter()
            .flat_map(|inc| inc.concept.iter().map(|c| c.code.as_str()))
            .collect();
        assert!(codes.contains(&"code1"));
        assert!(codes.contains(&"code2"));
        assert!(codes.contains(&"code3"));
    }

    #[test]
    fn test_disjunction_translation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("code1; code2; code3").unwrap();
        let compose = translator.translate(&expr).unwrap();

        // Disjunction with concepts should create one include with multiple concepts
        assert_eq!(compose.include.len(), 1);
        assert_eq!(compose.exclude.len(), 0);

        let include = &compose.include[0];
        assert_eq!(include.concept.len(), 3);

        let codes: Vec<&str> = include.concept.iter().map(|c| c.code.as_str()).collect();
        assert!(codes.contains(&"code1"));
        assert!(codes.contains(&"code2"));
        assert!(codes.contains(&"code3"));
    }

    #[test]
    fn test_disjunction_with_filters_creates_separate_includes() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("status = \"active\"; concept << 123456").unwrap();
        let compose = translator.translate(&expr).unwrap();

        // Disjunction with filters should create separate includes (OR logic)
        assert_eq!(compose.include.len(), 2);
        assert_eq!(compose.exclude.len(), 0);

        // First include should have status filter
        let first_include = &compose.include[0];
        assert_eq!(first_include.filter.len(), 1);
        assert_eq!(first_include.concept.len(), 0);
        assert_eq!(first_include.filter[0].property, "status");
        assert_eq!(first_include.filter[0].value, "active");

        // Second include should have concept filter
        let second_include = &compose.include[1];
        assert_eq!(second_include.filter.len(), 1);
        assert_eq!(second_include.concept.len(), 0);
        assert_eq!(second_include.filter[0].property, "concept");
        assert_eq!(second_include.filter[0].value, "123456");
    }

    #[test]
    fn test_conjunction_with_filters_creates_combined_filters() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("status = \"active\", concept << 123456").unwrap();
        let compose = translator.translate(&expr).unwrap();

        // Conjunction with filters should create single include with multiple filters (AND logic)
        assert_eq!(compose.include.len(), 1);
        assert_eq!(compose.exclude.len(), 0);

        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 2);
        assert_eq!(include.concept.len(), 0);

        // Should have both filters
        let filter_properties: Vec<&str> =
            include.filter.iter().map(|f| f.property.as_str()).collect();
        assert!(filter_properties.contains(&"status"));
        assert!(filter_properties.contains(&"concept"));
    }

    #[test]
    fn test_exclusion_translation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("* - inactive").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        assert_eq!(compose.exclude.len(), 1);

        // Include should be wildcard (all codes)
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));

        // Exclude should contain the excluded code
        let exclude = &compose.exclude[0];
        assert_eq!(exclude.concept.len(), 1);
        assert_eq!(exclude.concept[0].code, "inactive");
    }

    #[test]
    fn test_include_valueset() {
        let translator = VclTranslator::new();
        let expr = parse_vcl("^http://example.org/valueset").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.value_set.len(), 1);
        assert_eq!(include.value_set[0], "http://example.org/valueset");
    }

    #[test]
    fn test_system_uri_include_valueset() {
        let translator = VclTranslator::new();
        let expr = parse_vcl("^(http://snomed.info/sct)").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
    }

    #[test]
    fn test_nested_expression() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("(code1, code2)").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        // The nested conjunction should be flattened into concepts
        assert!(!include.concept.is_empty());
    }

    #[test]
    fn test_complex_expression() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr =
            parse_vcl("(http://snomed.info/sct)status = \"active\", category << 123456").unwrap();
        let compose = translator.translate(&expr).unwrap();

        // Should create one include combining the conjunction concepts/filters
        assert_eq!(compose.include.len(), 1);

        // The combined include should have both status and category filters
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert!(include.filter.iter().any(|f| f.property == "status"));
        assert!(include.filter.iter().any(|f| f.property == "category"));
    }

    #[test]
    fn test_of_operation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("*.category").unwrap();
        let result = translator.translate(&expr);

        // Of operations should not be translatable to FHIR compose
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Of operations"));
        assert!(error.to_string().contains("represent value sets"));
    }

    #[test]
    fn test_code_list_with_property_uses_in_operator() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("{123456, 789012, 345678}.status").unwrap();
        let result = translator.translate(&expr);

        // Code list with property access should not be translatable to FHIR compose
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Of operations"));
        assert!(error.to_string().contains("represent value sets"));
    }

    #[test]
    fn test_single_code_with_property_uses_equals_operator() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("123456.status").unwrap();
        let result = translator.translate(&expr);

        // Single code with property access should not be translatable to FHIR compose
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Of operations"));
        assert!(error.to_string().contains("represent value sets"));
    }

    #[test]
    fn test_translation_validation_examples() {
        use crate::parse_vcl;
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());

        // TRANSLATABLE: Simple codes
        assert!(translator.translate(&parse_vcl("123456").unwrap()).is_ok());

        // TRANSLATABLE: Property filters with operators
        assert!(translator
            .translate(&parse_vcl("status = \"active\"").unwrap())
            .is_ok());
        assert!(translator
            .translate(&parse_vcl("concept << 123456").unwrap())
            .is_ok());

        // TRANSLATABLE: Conjunctions/disjunctions of valid expressions
        assert!(translator
            .translate(&parse_vcl("(123456, 789012)").unwrap())
            .is_ok());
        assert!(translator
            .translate(&parse_vcl("status = \"active\"; concept << 123456").unwrap())
            .is_ok());

        // NOT TRANSLATABLE: Of operations (value set expressions)
        assert!(translator
            .translate(&parse_vcl("*.concept").unwrap())
            .is_err());
        assert!(translator
            .translate(&parse_vcl("B.codeprop").unwrap())
            .is_err());
        assert!(translator
            .translate(&parse_vcl("123456.status").unwrap())
            .is_err());
        assert!(translator
            .translate(&parse_vcl("{123456, 789012}.status").unwrap())
            .is_err());
    }

    #[test]
    fn test_filter_list_translation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("{status=\"active\", category<<12345}").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 2);

        // First filter: status = "active"
        let filter1 = &include.filter[0];
        assert_eq!(filter1.property, "status");
        assert_eq!(filter1.op, fhir::FilterOperator::Equal);
        assert_eq!(filter1.value, "active");

        // Second filter: category << 12345
        let filter2 = &include.filter[1];
        assert_eq!(filter2.property, "category");
        assert_eq!(filter2.op, fhir::FilterOperator::IsA);
        assert_eq!(filter2.value, "12345");
    }

    #[test]
    fn test_simple_filter_list() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("{has_ingredient=1886, has_dose_form=317541}").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 2);

        // First filter: has_ingredient = 1886
        let filter1 = &include.filter[0];
        assert_eq!(filter1.property, "has_ingredient");
        assert_eq!(filter1.op, fhir::FilterOperator::Equal);
        assert_eq!(filter1.value, "1886");

        // Second filter: has_dose_form = 317541
        let filter2 = &include.filter[1];
        assert_eq!(filter2.property, "has_dose_form");
        assert_eq!(filter2.op, fhir::FilterOperator::Equal);
        assert_eq!(filter2.value, "317541");
    }

    #[test]
    fn test_parentheses_filter_conjunction() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("(has_ingredient=1886, has_dose_form=317541)").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 2);

        // First filter: has_ingredient = 1886
        let filter1 = &include.filter[0];
        assert_eq!(filter1.property, "has_ingredient");
        assert_eq!(filter1.op, fhir::FilterOperator::Equal);
        assert_eq!(filter1.value, "1886");

        // Second filter: has_dose_form = 317541
        let filter2 = &include.filter[1];
        assert_eq!(filter2.property, "has_dose_form");
        assert_eq!(filter2.op, fhir::FilterOperator::Equal);
        assert_eq!(filter2.value, "317541");
    }

    #[test]
    fn test_complex_nested_filter_list() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("{has_ingredient=1886, has_dose_form^{concept<<1151133}}").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 2);

        // First filter: has_ingredient = 1886
        let filter1 = &include.filter[0];
        assert_eq!(filter1.property, "has_ingredient");
        assert_eq!(filter1.op, fhir::FilterOperator::Equal);
        assert_eq!(filter1.value, "1886");

        // Second filter: has_dose_form << 1151133 (transformed from ^{concept<<1151133})
        let filter2 = &include.filter[1];
        assert_eq!(filter2.property, "has_dose_form");
        assert_eq!(filter2.op, fhir::FilterOperator::IsA); // Transformed to is-a
        assert_eq!(filter2.value, "1151133"); // Just the value from the nested filter
    }

    #[test]
    fn test_deeply_nested_filter_error() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("consists_of^{has_ingredient^{has_tradename=2201670}}").unwrap();
        let result = translator.translate(&expr);

        // Should return an error for deeply nested filters
        assert!(result.is_err());
        let error_msg = result.err().unwrap().to_string();
        assert!(error_msg.contains("Complex nested filters are not yet implemented"));
    }

    #[test]
    fn test_convenience_function() {
        let compose = translate_vcl_string_to_fhir("(http://snomed.info/sct)123456").unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.system, Some("http://snomed.info/sct".to_string()));
        assert_eq!(include.concept.len(), 1);
        assert_eq!(include.concept[0].code, "123456");
    }

    #[test]
    fn test_filter_operator_mapping() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());

        let test_cases = vec![
            ("prop = value", fhir::FilterOperator::Equal),
            ("prop << value", fhir::FilterOperator::IsA),
            ("prop ~<< value", fhir::FilterOperator::IsNotA),
            ("prop < value", fhir::FilterOperator::DescendentOf),
            ("prop ^ value", fhir::FilterOperator::In),
            ("prop ~^ value", fhir::FilterOperator::NotIn),
            ("prop >> value", fhir::FilterOperator::Generalizes),
            ("prop <! value", fhir::FilterOperator::ChildOf),
            ("prop !!< value", fhir::FilterOperator::DescendentLeaf),
            ("prop ? value", fhir::FilterOperator::Exists),
        ];

        for (vcl_expr, expected_op) in test_cases {
            let expr = parse_vcl(vcl_expr).unwrap();
            let compose = translator.translate(&expr).unwrap();

            assert_eq!(compose.include.len(), 1);
            let include = &compose.include[0];
            assert_eq!(include.filter.len(), 1);
            assert_eq!(
                include.filter[0].op, expected_op,
                "Failed for expression: {vcl_expr}"
            );
        }
    }

    #[test]
    fn test_empty_compose() {
        let compose = ValueSetCompose::new();
        assert!(compose.is_empty());

        let mut compose = ValueSetCompose::new();
        compose.add_include(ValueSetInclude::new_system(
            "http://example.org".to_string(),
        ));
        assert!(!compose.is_empty());
    }

    #[test]
    fn test_json_serialization() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("status = \"active\"").unwrap();
        let compose = translator.translate(&expr).unwrap();

        let json = serde_json::to_string_pretty(&compose).unwrap();
        assert!(json.contains("status"));
        assert!(json.contains("active"));
        assert!(json.contains("include"));

        // Test deserialization
        let deserialized: ValueSetCompose = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, compose);
    }
}
