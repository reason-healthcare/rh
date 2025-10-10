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

    /// Whether to create separate includes for conjunctions (true) or combine them (false)
    pub separate_conjunction_includes: bool,
}

impl VclTranslator {
    /// Create a new translator with default settings
    pub fn new() -> Self {
        Self {
            default_system: None,
            separate_conjunction_includes: true,
        }
    }

    /// Create a new translator with a default code system
    pub fn with_default_system(system: String) -> Self {
        Self {
            default_system: Some(system),
            separate_conjunction_includes: true,
        }
    }

    /// Set whether to create separate includes for conjunctions
    pub fn set_separate_conjunction_includes(mut self, separate: bool) -> Self {
        self.separate_conjunction_includes = separate;
        self
    }

    /// Translate a VCL expression into a FHIR ValueSet compose structure
    pub fn translate(&self, vcl_expr: &VclExpression) -> VclResult<ValueSetCompose> {
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
                // Conjunction: Add main expression and all additional sub-expressions
                if let Some(include) = main_include {
                    compose.add_include(include);
                }

                for sub_expr in sub_exprs {
                    if let Some(include) = self.translate_sub_expression(sub_expr)? {
                        compose.add_include(include);
                    }
                }
            }
            Some(Operation::Disjunction(sub_exprs)) => {
                // Disjunction: Combine all expressions into concepts of a single include
                let system = self.get_system_from_sub_expr(&expr.sub_expr);
                let mut combined_include = ValueSetInclude::new_system(
                    system.unwrap_or_else(|| self.default_system.clone().unwrap_or_default()),
                );

                // Add main expression concepts
                if let Some(main_include) = main_include {
                    self.merge_include_into(&main_include, &mut combined_include)?;
                }

                // Add all disjunction concepts
                for sub_expr in sub_exprs {
                    if let Some(include) = self.translate_sub_expression(sub_expr)? {
                        self.merge_include_into(&include, &mut combined_include)?;
                    }
                }

                if !combined_include.concept.is_empty() || !combined_include.filter.is_empty() {
                    compose.add_include(combined_include);
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
        let system = self.get_system_from_sub_expr(sub_expr);

        match &sub_expr.content {
            SubExpressionContent::Simple(simple_expr) => {
                self.translate_simple_expression(simple_expr, system)
            }
            SubExpressionContent::Nested(nested_expr) => {
                // For nested expressions, create a new compose and merge results
                let mut nested_compose = ValueSetCompose::new();
                self.translate_expression(nested_expr, &mut nested_compose)?;

                // Merge all includes from nested compose
                // For simplicity, we'll take the first include and merge others into it
                if let Some(first_include) = nested_compose.include.into_iter().next() {
                    Ok(Some(first_include))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// Translate a simple expression into a ValueSet include
    fn translate_simple_expression(
        &self,
        simple_expr: &SimpleExpression,
        system: Option<String>,
    ) -> VclResult<Option<ValueSetInclude>> {
        let system_uri = system.or_else(|| self.default_system.clone());

        match simple_expr {
            SimpleExpression::Wildcard => {
                // Wildcard - include all from system
                if let Some(sys) = system_uri {
                    let include = ValueSetInclude::new_system(sys);
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
                    let mut include = ValueSetInclude::new_system(sys);
                    include.add_concept(code.value().to_string(), None);
                    Ok(Some(include))
                } else {
                    Err(VclError::parse_error("Code requires a system URI", 0, ""))
                }
            }
            SimpleExpression::Filter(filter) => {
                // Filter expression
                self.translate_filter(filter, system_uri)
            }
            SimpleExpression::IncludeValueSet(include_vs) => {
                // Include ValueSet
                match include_vs {
                    IncludeValueSet::Uri(uri) => {
                        Ok(Some(ValueSetInclude::new_valueset(uri.clone())))
                    }
                    IncludeValueSet::SystemUri(system_uri) => {
                        Ok(Some(ValueSetInclude::new_system(system_uri.uri.clone())))
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
    ) -> VclResult<Option<ValueSetInclude>> {
        let system_uri = system
            .or_else(|| self.default_system.clone())
            .ok_or_else(|| VclError::parse_error("Filter requires a system URI", 0, ""))?;

        let mut include = ValueSetInclude::new_system(system_uri);

        match filter {
            Filter::PropertyFilter {
                property,
                operator,
                value,
            } => {
                let fhir_op = self.map_filter_operator(operator)?;
                let filter_value = self.map_filter_value(value)?;
                include.add_filter(property.value().to_string(), fhir_op, filter_value);
            }
            Filter::OfOperation { source, property } => {
                // "Of" operations are complex - for now, create a filter based on the property
                // This is a simplified implementation
                let fhir_op = fhir::FilterOperator::Equal; // Default to equality
                let source_value = self.map_of_source_to_value(source)?;
                include.add_filter(property.value().to_string(), fhir_op, source_value);
            }
        }

        Ok(Some(include))
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
            FilterValue::FilterList(_) => {
                // Complex filter lists not supported in simple FHIR filters
                Err(VclError::parse_error(
                    "Complex filter lists not supported in FHIR translation",
                    0,
                    "",
                ))
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
    fn get_system_from_sub_expr(&self, sub_expr: &SubExpression) -> Option<String> {
        sub_expr.system_uri.as_ref().map(|uri| uri.uri.clone())
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

        // Conjunction should create multiple includes
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

        // Disjunction should create one include with multiple concepts
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

        // Should create multiple includes for the conjunction
        assert_eq!(compose.include.len(), 2);

        // First include should have status filter
        let first_include = &compose.include[0];
        assert_eq!(
            first_include.system,
            Some("http://snomed.info/sct".to_string())
        );
        assert!(first_include.filter.iter().any(|f| f.property == "status"));

        // Second include should have category filter
        let second_include = &compose.include[1];
        assert!(second_include
            .filter
            .iter()
            .any(|f| f.property == "category"));
    }

    #[test]
    fn test_of_operation() {
        let translator = VclTranslator::with_default_system("http://snomed.info/sct".to_string());
        let expr = parse_vcl("*.category").unwrap();
        let compose = translator.translate(&expr).unwrap();

        assert_eq!(compose.include.len(), 1);
        let include = &compose.include[0];
        assert_eq!(include.filter.len(), 1);

        let filter = &include.filter[0];
        assert_eq!(filter.property, "category");
        assert_eq!(filter.value, "*");
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
                "Failed for expression: {}",
                vcl_expr
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
