//! VCL Expression Explainer
//!
//! Provides human-readable explanations of VCL expressions, distinguishing between
//! filter expressions (for codes) and value set expressions (for values).

use crate::ast::{
    Expression, Filter, FilterOperator, FilterValue, IncludeValueSet, OfSource, Operation,
    SimpleExpression, SubExpression, SubExpressionContent, SystemUri, VclExpression,
};
use crate::error::{VclError, VclResult};
use serde::{Deserialize, Serialize};

/// Result of explaining a VCL expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationResult {
    /// The original VCL expression
    pub expression: String,
    /// Plain English explanation
    pub explanation: String,
    /// Type of expression (filter vs value set)
    pub expression_type: ExpressionType,
    /// Whether this can be translated to FHIR compose
    pub translatable_to_fhir: bool,
    /// Semantic components breakdown
    pub components: Vec<ComponentExplanation>,
}

/// Type of VCL expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionType {
    /// Filter expression - defines criteria for selecting codes
    Filter { description: String },
    /// Value set expression - represents a collection of values
    ValueSet { description: String },
    /// Mixed expression containing both types
    Mixed { description: String },
}

/// Explanation of a component within the expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentExplanation {
    /// The component text
    pub component: String,
    /// What this component means
    pub meaning: String,
    /// Type of this component
    pub component_type: String,
    /// Nesting level for hierarchical display (0 = root level)
    pub nesting_level: usize,
}

/// VCL Expression Explainer
#[derive(Debug, Clone)]
pub struct VclExplainer {
    /// Default system for context
    pub default_system: Option<String>,
}

impl VclExplainer {
    /// Create a new explainer
    pub fn new() -> Self {
        Self {
            default_system: None,
        }
    }

    /// Create explainer with default system
    pub fn with_default_system(system: String) -> Self {
        Self {
            default_system: Some(system),
        }
    }

    /// Explain a VCL expression
    pub fn explain(&self, vcl_expr: &VclExpression) -> VclResult<ExplanationResult> {
        self.explain_with_text(vcl_expr, "<expression>")
    }

    /// Explain a VCL expression with original text
    pub fn explain_with_text(
        &self,
        vcl_expr: &VclExpression,
        original_text: &str,
    ) -> VclResult<ExplanationResult> {
        let (explanation, expr_type, components) = self.explain_expression(&vcl_expr.expr)?;

        let translatable = self.is_translatable_to_fhir(&vcl_expr.expr);

        Ok(ExplanationResult {
            expression: original_text.to_string(),
            explanation,
            expression_type: expr_type,
            translatable_to_fhir: translatable,
            components,
        })
    }

    /// Check if expression is translatable to FHIR
    fn is_translatable_to_fhir(&self, expr: &Expression) -> bool {
        // Use the same logic as the translator validation
        match self.check_expression_translatable(expr) {
            Ok(()) => true,
            Err(_) => false,
        }
    }

    /// Check if expression is translatable (same logic as translator)
    fn check_expression_translatable(&self, expr: &Expression) -> VclResult<()> {
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

    /// Check if sub-expression is translatable
    fn check_sub_expression_translatable(&self, sub_expr: &SubExpression) -> VclResult<()> {
        match &sub_expr.content {
            SubExpressionContent::Simple(simple) => {
                self.check_simple_expression_translatable(simple)?;
            }
            SubExpressionContent::Nested(nested_expr) => {
                self.check_expression_translatable(nested_expr)?;
            }
        }
        Ok(())
    }

    /// Check if simple expression is translatable
    fn check_simple_expression_translatable(&self, simple: &SimpleExpression) -> VclResult<()> {
        match simple {
            SimpleExpression::Wildcard => Ok(()),
            SimpleExpression::Code(_) => Ok(()),
            SimpleExpression::IncludeValueSet(_) => Ok(()),
            SimpleExpression::Filter(filter) => match filter {
                Filter::PropertyFilter { .. } => Ok(()),
                Filter::OfOperation { .. } => Err(VclError::TranslationError {
                    message: "Of operations represent value sets".to_string(),
                }),
            },
            SimpleExpression::FilterList(filters) => {
                for filter in filters {
                    if matches!(filter, Filter::OfOperation { .. }) {
                        return Err(VclError::TranslationError {
                            message: "Of operations represent value sets".to_string(),
                        });
                    }
                }
                Ok(())
            }
        }
    }

    /// Explain an expression
    fn explain_expression(
        &self,
        expr: &Expression,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        self.explain_expression_with_level(expr, 0)
    }

    /// Explain an expression with a specific nesting level
    fn explain_expression_with_level(
        &self,
        expr: &Expression,
        level: usize,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        let mut components = Vec::new();

        // Explain main sub-expression
        let (main_explanation, main_type, main_components) =
            self.explain_sub_expression_with_level(&expr.sub_expr, level)?;
        components.extend(main_components);

        // Handle operations
        if let Some(ref operation) = expr.operation {
            let (op_explanation, op_components) =
                self.explain_operation_with_level(operation, &main_type, level)?;
            components.extend(op_components);

            let full_explanation = format!("{main_explanation} {op_explanation}");
            let expr_type = self.determine_combined_type(&main_type, operation);

            return Ok((full_explanation, expr_type, components));
        }

        Ok((main_explanation, main_type, components))
    }

    /// Explain a sub-expression
    fn explain_sub_expression(
        &self,
        sub_expr: &SubExpression,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        self.explain_sub_expression_with_level(sub_expr, 0)
    }

    /// Explain a sub-expression with a specific nesting level
    fn explain_sub_expression_with_level(
        &self,
        sub_expr: &SubExpression,
        level: usize,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        let mut explanation = String::new();
        let mut components = Vec::new();

        // Handle system URI
        if let Some(ref system_uri) = sub_expr.system_uri {
            let system_text = self.explain_system_uri(system_uri);
            explanation.push_str(&format!("From {system_text} system: "));
            components.push(ComponentExplanation {
                component: format!("({})", system_uri.uri),
                meaning: format!("Codes from the {system_text} terminology system"),
                component_type: "System URI".to_string(),
                nesting_level: level,
            });
        }

        // Handle content
        let (content_explanation, expr_type, content_components) = match &sub_expr.content {
            SubExpressionContent::Simple(simple) => {
                self.explain_simple_expression_with_level(simple, level)?
            }
            SubExpressionContent::Nested(nested) => {
                self.explain_expression_with_level(nested, level + 1)?
            }
        };

        explanation.push_str(&content_explanation);
        components.extend(content_components);

        Ok((explanation, expr_type, components))
    }

    /// Explain a simple expression with a specific nesting level
    fn explain_simple_expression_with_level(
        &self,
        simple: &SimpleExpression,
        level: usize,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        let _components: Vec<ComponentExplanation> = Vec::new();

        match simple {
            SimpleExpression::Wildcard => Ok((
                "all codes".to_string(),
                ExpressionType::Filter {
                    description: "Selects all codes from the code system".to_string(),
                },
                vec![ComponentExplanation {
                    component: "*".to_string(),
                    meaning: "All codes/concepts in the system".to_string(),
                    component_type: "Wildcard".to_string(),
                    nesting_level: level,
                }],
            )),
            SimpleExpression::Code(code) => {
                let code_text = code.value();
                Ok((
                    format!("code '{code_text}'"),
                    ExpressionType::Filter {
                        description: format!("Selects the specific code '{code_text}'"),
                    },
                    vec![ComponentExplanation {
                        component: code_text.to_string(),
                        meaning: format!("The specific code/concept '{code_text}'"),
                        component_type: "Code".to_string(),
                        nesting_level: level,
                    }],
                ))
            }
            SimpleExpression::Filter(filter) => self.explain_filter_with_level(filter, level),
            SimpleExpression::FilterList(filters) => {
                self.explain_filter_list_with_level(filters, level)
            }
            SimpleExpression::IncludeValueSet(vs) => {
                self.explain_include_valueset_with_level(vs, level)
            }
        }
    }

    /// Explain a filter with a specific nesting level
    fn explain_filter_with_level(
        &self,
        filter: &Filter,
        level: usize,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        match filter {
            Filter::PropertyFilter {
                property,
                operator,
                value,
            } => {
                let prop_name = property.value();
                let op_text = self.explain_operator(operator);
                let value_text = self.explain_filter_value(value);

                // Special handling for nested filters with "In" operator
                let explanation = match (operator, value) {
                    (FilterOperator::In, FilterValue::FilterList(filters)) => {
                        if filters.len() == 1 {
                            let nested_explanation = self.explain_single_filter(&filters[0]);
                            format!(
                                "codes whose '{prop_name}' is a concept whose {nested_explanation}"
                            )
                        } else {
                            let filter_explanations: Vec<String> = filters
                                .iter()
                                .map(|filter| self.explain_single_filter(filter))
                                .collect();
                            format!(
                                "codes whose '{}' is a concept whose ({})",
                                prop_name,
                                filter_explanations.join(" AND ")
                            )
                        }
                    }
                    _ => format!("codes where {prop_name} {op_text} {value_text}"),
                };

                Ok((
                    explanation.clone(),
                    ExpressionType::Filter {
                        description: format!(
                            "Filters codes based on property criteria: {explanation}"
                        ),
                    },
                    vec![
                        ComponentExplanation {
                            component: prop_name.to_string(),
                            meaning: format!("The '{prop_name}' property of codes"),
                            component_type: "Property".to_string(),
                            nesting_level: level,
                        },
                        ComponentExplanation {
                            component: format!("{operator:?}"),
                            meaning: op_text,
                            component_type: "Operator".to_string(),
                            nesting_level: level,
                        },
                        ComponentExplanation {
                            component: value_text.clone(),
                            meaning: format!("The target value {value_text}"),
                            component_type: "Value".to_string(),
                            nesting_level: level,
                        },
                    ],
                ))
            }
            Filter::OfOperation { source, property } => {
                let source_text = self.explain_of_source(source)?;
                let prop_name = property.value();

                let explanation = format!("the set of {prop_name} values from {source_text}");

                Ok((
                    explanation.clone(),
                    ExpressionType::ValueSet {
                        description: format!("Represents a collection of values: {explanation}"),
                    },
                    vec![
                        ComponentExplanation {
                            component: source_text.clone(),
                            meaning: format!("The source: {source_text}"),
                            component_type: "Source".to_string(),
                            nesting_level: level,
                        },
                        ComponentExplanation {
                            component: prop_name.to_string(),
                            meaning: format!("The '{prop_name}' property values"),
                            component_type: "Property".to_string(),
                            nesting_level: level,
                        },
                    ],
                ))
            }
        }
    }

    /// Explain filter list with a specific nesting level
    fn explain_filter_list_with_level(
        &self,
        filters: &[Filter],
        level: usize,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        if filters.is_empty() {
            return Ok((
                "no filters".to_string(),
                ExpressionType::Filter {
                    description: "Empty filter list".to_string(),
                },
                vec![],
            ));
        }

        let mut explanations = Vec::new();
        let mut all_components = Vec::new();
        let mut has_value_sets = false;

        for filter in filters {
            let (explanation, expr_type, components) =
                self.explain_filter_with_level(filter, level)?;
            explanations.push(explanation);
            all_components.extend(components);

            if matches!(expr_type, ExpressionType::ValueSet { .. }) {
                has_value_sets = true;
            }
        }

        let combined_explanation = explanations.join(" AND ");

        let expr_type = if has_value_sets {
            ExpressionType::Mixed {
                description: "Contains both filter criteria and value set expressions".to_string(),
            }
        } else {
            ExpressionType::Filter {
                description: format!("Multiple filter criteria combined: {combined_explanation}"),
            }
        };

        Ok((
            format!("codes where {combined_explanation}"),
            expr_type,
            all_components,
        ))
    }

    /// Explain include valueset with a specific nesting level
    fn explain_include_valueset_with_level(
        &self,
        vs: &IncludeValueSet,
        level: usize,
    ) -> VclResult<(String, ExpressionType, Vec<ComponentExplanation>)> {
        match vs {
            IncludeValueSet::Uri(uri) => Ok((
                format!("all codes from ValueSet '{uri}'"),
                ExpressionType::Filter {
                    description: format!("Includes all codes from the ValueSet '{uri}'"),
                },
                vec![ComponentExplanation {
                    component: format!("^{uri}"),
                    meaning: format!("Reference to ValueSet '{uri}'"),
                    component_type: "ValueSet Reference".to_string(),
                    nesting_level: level,
                }],
            )),
            IncludeValueSet::SystemUri(system_uri) => {
                let system_text = self.explain_system_uri(system_uri);
                Ok((
                    format!("all codes from {system_text} system"),
                    ExpressionType::Filter {
                        description: format!("Includes all codes from the {system_text} system"),
                    },
                    vec![ComponentExplanation {
                        component: format!("^({})", system_uri.uri),
                        meaning: format!("All codes from {system_text} system"),
                        component_type: "System Reference".to_string(),
                        nesting_level: level,
                    }],
                ))
            }
        }
    }

    /// Explain an operation with a specific nesting level
    fn explain_operation_with_level(
        &self,
        operation: &Operation,
        main_type: &ExpressionType,
        level: usize,
    ) -> VclResult<(String, Vec<ComponentExplanation>)> {
        match operation {
            Operation::Conjunction(sub_exprs) => {
                let is_filter = matches!(main_type, ExpressionType::Filter { .. });
                let mut components = Vec::new();
                let mut explanations = Vec::new();

                // Add the conjunction operator explanation
                components.push(ComponentExplanation {
                    component: ",".to_string(),
                    meaning: if is_filter {
                        "AND - all conditions must be true".to_string()
                    } else {
                        "Combined collection of values".to_string()
                    },
                    component_type: "Conjunction".to_string(),
                    nesting_level: level,
                });

                // Explain each sub-expression
                for sub_expr in sub_exprs {
                    let (sub_explanation, _sub_type, sub_components) =
                        self.explain_sub_expression_with_level(sub_expr, level + 1)?;
                    explanations.push(sub_explanation);
                    components.extend(sub_components);
                }

                let full_explanation = if is_filter {
                    format!("AND {}", explanations.join(", AND "))
                } else {
                    format!("combined with {}", explanations.join(", and "))
                };

                Ok((full_explanation, components))
            }
            Operation::Disjunction(sub_exprs) => {
                let is_filter = matches!(main_type, ExpressionType::Filter { .. });
                let mut components = Vec::new();
                let mut explanations = Vec::new();

                // Add the disjunction operator explanation
                components.push(ComponentExplanation {
                    component: ";".to_string(),
                    meaning: if is_filter {
                        "OR - any condition can be true".to_string()
                    } else {
                        "Combined collection of values".to_string()
                    },
                    component_type: "Disjunction".to_string(),
                    nesting_level: level,
                });

                // Explain each sub-expression
                for sub_expr in sub_exprs {
                    let (sub_explanation, _sub_type, sub_components) =
                        self.explain_sub_expression_with_level(sub_expr, level + 1)?;
                    explanations.push(sub_explanation);
                    components.extend(sub_components);
                }

                let full_explanation = if is_filter {
                    format!("OR {}", explanations.join(", OR "))
                } else {
                    format!("combined with {}", explanations.join(", and "))
                };

                Ok((full_explanation, components))
            }
            Operation::Exclusion(excluded_expr) => {
                // Explain what is being excluded
                let (excluded_explanation, _excluded_type, excluded_components) =
                    self.explain_sub_expression_with_level(excluded_expr, level + 1)?;

                let exclusion_text = format!("EXCLUDING {excluded_explanation}");

                let mut components = vec![ComponentExplanation {
                    component: "-".to_string(),
                    meaning: "EXCLUDE - remove matching items".to_string(),
                    component_type: "Exclusion".to_string(),
                    nesting_level: level,
                }];

                // Add the excluded expression components
                components.extend(excluded_components);

                Ok((exclusion_text, components))
            }
        }
    }

    /// Determine combined expression type
    fn determine_combined_type(
        &self,
        main_type: &ExpressionType,
        operation: &Operation,
    ) -> ExpressionType {
        match operation {
            Operation::Conjunction(sub_exprs) | Operation::Disjunction(sub_exprs) => {
                // Analyze all sub-expressions to create comprehensive description
                let mut descriptions = Vec::new();
                let mut has_filters = false;
                let mut has_value_sets = false;

                // Add main type description
                match main_type {
                    ExpressionType::Filter { description } => {
                        descriptions.push(description.clone());
                        has_filters = true;
                    }
                    ExpressionType::ValueSet { description } => {
                        descriptions.push(description.clone());
                        has_value_sets = true;
                    }
                    ExpressionType::Mixed { .. } => {
                        has_filters = true;
                        has_value_sets = true;
                    }
                }

                // Analyze each sub-expression
                for sub_expr in sub_exprs {
                    if let Ok((_, sub_type, _)) = self.explain_sub_expression(sub_expr) {
                        match sub_type {
                            ExpressionType::Filter { description } => {
                                descriptions.push(description);
                                has_filters = true;
                            }
                            ExpressionType::ValueSet { description } => {
                                descriptions.push(description);
                                has_value_sets = true;
                            }
                            ExpressionType::Mixed { .. } => {
                                has_filters = true;
                                has_value_sets = true;
                            }
                        }
                    }
                }

                // Determine final type based on what we found
                let connector = match operation {
                    Operation::Conjunction(_) => " AND ",
                    Operation::Disjunction(_) => " OR ",
                    _ => " ",
                };

                let combined_description = descriptions.join(connector);

                if has_filters && has_value_sets {
                    ExpressionType::Mixed {
                        description: format!("Mixed expression: {combined_description}"),
                    }
                } else if has_filters {
                    ExpressionType::Filter {
                        description: combined_description,
                    }
                } else {
                    ExpressionType::ValueSet {
                        description: combined_description,
                    }
                }
            }
            Operation::Exclusion(_) => {
                // For exclusions, keep the main type but note the exclusion
                match main_type {
                    ExpressionType::Filter { description } => ExpressionType::Filter {
                        description: format!("{description} with exclusions"),
                    },
                    ExpressionType::ValueSet { description } => ExpressionType::ValueSet {
                        description: format!("{description} with exclusions"),
                    },
                    ExpressionType::Mixed { description } => ExpressionType::Mixed {
                        description: format!("{description} with exclusions"),
                    },
                }
            }
        }
    }

    /// Explain system URI
    fn explain_system_uri(&self, system_uri: &SystemUri) -> String {
        let system_name = self.get_system_name(&system_uri.uri);
        if let Some(ref version) = system_uri.version {
            format!("{system_name} (version {version})")
        } else {
            system_name
        }
    }

    /// Get friendly name for system URI
    fn get_system_name(&self, uri: &str) -> String {
        match uri {
            "http://snomed.info/sct" => "SNOMED CT".to_string(),
            "http://loinc.org" => "LOINC".to_string(),
            "http://hl7.org/fhir/sid/icd-10-cm" => "ICD-10-CM".to_string(),
            "http://hl7.org/fhir/sid/icd-10" => "ICD-10".to_string(),
            _ => uri.to_string(),
        }
    }

    /// Explain filter operator
    fn explain_operator(&self, operator: &FilterOperator) -> String {
        match operator {
            FilterOperator::Equals => "equals".to_string(),
            FilterOperator::IsA => "is a type of".to_string(),
            FilterOperator::IsNotA => "is not a type of".to_string(),
            FilterOperator::DescendantOf => "is a descendant of".to_string(),
            FilterOperator::Regex => "matches pattern".to_string(),
            FilterOperator::In => "is in the list".to_string(),
            FilterOperator::NotIn => "is not in the list".to_string(),
            FilterOperator::Generalizes => "generalizes".to_string(),
            FilterOperator::ChildOf => "is a direct child of".to_string(),
            FilterOperator::DescendantLeaf => "is a leaf descendant of".to_string(),
            FilterOperator::Exists => "exists".to_string(),
        }
    }

    /// Explain filter value
    fn explain_filter_value(&self, value: &FilterValue) -> String {
        match value {
            FilterValue::Code(code) => format!("'{}'", code.value()),
            FilterValue::String(s) => format!("\"{s}\""),
            FilterValue::CodeList(codes) => {
                let code_values: Vec<String> =
                    codes.iter().map(|c| format!("'{}'", c.value())).collect();
                format!("({})", code_values.join(", "))
            }
            FilterValue::Uri(uri) => format!("<{uri}>"),
            FilterValue::FilterList(filters) => {
                // Explain nested filters
                let filter_explanations: Vec<String> = filters
                    .iter()
                    .map(|filter| self.explain_single_filter(filter))
                    .collect();

                if filter_explanations.len() == 1 {
                    filter_explanations[0].clone()
                } else {
                    format!("({})", filter_explanations.join(" AND "))
                }
            }
        }
    }

    /// Explain a single filter without context
    fn explain_single_filter(&self, filter: &Filter) -> String {
        match filter {
            Filter::PropertyFilter {
                property,
                operator,
                value,
            } => {
                let prop_name = property.value();

                // Special handling for nested filters with "In" operator
                match (operator, value) {
                    (FilterOperator::In, FilterValue::FilterList(filters)) => {
                        if filters.len() == 1 {
                            let nested_explanation = self.explain_single_filter(&filters[0]);
                            format!("'{prop_name}' is a concept whose {nested_explanation}")
                        } else {
                            let filter_explanations: Vec<String> = filters
                                .iter()
                                .map(|filter| self.explain_single_filter(filter))
                                .collect();
                            format!(
                                "'{}' is a concept whose ({})",
                                prop_name,
                                filter_explanations.join(" AND ")
                            )
                        }
                    }
                    _ => {
                        let operator_text = self.explain_operator_nested(operator);
                        let value_text = self.explain_filter_value_nested(value);
                        format!("'{prop_name}' {operator_text} {value_text}")
                    }
                }
            }
            Filter::OfOperation { source, property } => match self.explain_of_source(source) {
                Ok(source_text) => format!("{}.{}", source_text, property.value()),
                Err(_) => format!("?.{}", property.value()),
            },
        }
    }

    /// Explain filter operator for nested contexts
    fn explain_operator_nested(&self, operator: &FilterOperator) -> String {
        match operator {
            FilterOperator::Equals => "equals".to_string(),
            FilterOperator::IsA => "is a type of".to_string(),
            FilterOperator::IsNotA => "is not a type of".to_string(),
            FilterOperator::DescendantOf => "is a descendant of".to_string(),
            FilterOperator::Regex => "matches pattern".to_string(),
            FilterOperator::In => "is in the list".to_string(),
            FilterOperator::NotIn => "is not in the list".to_string(),
            FilterOperator::Generalizes => "generalizes".to_string(),
            FilterOperator::ChildOf => "is a direct child of".to_string(),
            FilterOperator::DescendantLeaf => "is a leaf descendant of".to_string(),
            FilterOperator::Exists => "exists".to_string(),
        }
    }

    /// Explain filter value for nested contexts (no quotes around numeric codes)
    fn explain_filter_value_nested(&self, value: &FilterValue) -> String {
        match value {
            FilterValue::Code(code) => {
                // Check if it's a numeric code - if so, don't quote it
                let code_val = code.value();
                if code_val.chars().all(|c| c.is_ascii_digit()) {
                    code_val.to_string()
                } else {
                    format!("'{code_val}'")
                }
            }
            FilterValue::String(s) => format!("\"{s}\""),
            FilterValue::CodeList(codes) => {
                let code_values: Vec<String> = codes
                    .iter()
                    .map(|c| {
                        let code_val = c.value();
                        if code_val.chars().all(|c| c.is_ascii_digit()) {
                            code_val.to_string()
                        } else {
                            format!("'{code_val}'")
                        }
                    })
                    .collect();
                format!("({})", code_values.join(", "))
            }
            FilterValue::Uri(uri) => format!("<{uri}>"),
            FilterValue::FilterList(filters) => {
                // Explain nested filters
                let filter_explanations: Vec<String> = filters
                    .iter()
                    .map(|filter| self.explain_single_filter(filter))
                    .collect();

                if filter_explanations.len() == 1 {
                    filter_explanations[0].clone()
                } else {
                    format!("({})", filter_explanations.join(" AND "))
                }
            }
        }
    }

    /// Explain of source
    fn explain_of_source(&self, source: &OfSource) -> VclResult<String> {
        match source {
            OfSource::Wildcard => Ok("all codes".to_string()),
            OfSource::Code(code) => Ok(format!("code '{}'", code.value())),
            OfSource::CodeList(codes) => Ok(format!("{} specific codes", codes.len())),
            OfSource::Uri(uri) => Ok(format!("ValueSet '{uri}'")),
            OfSource::FilterList(filters) => {
                Ok(format!("codes matching {} filters", filters.len()))
            }
        }
    }
}

impl Default for VclExplainer {
    fn default() -> Self {
        Self::new()
    }
}
