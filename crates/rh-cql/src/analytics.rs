//! Inspectable CQL analytics planning helpers.

use crate::elm::{Expression, Library, StatementDef};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// Summary of an ELM library for humans and tools.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElmInspection {
    /// Library name.
    pub library: Option<String>,
    /// Library version.
    pub version: Option<String>,
    /// Model declarations.
    pub usings: Vec<UsingSummary>,
    /// Included CQL libraries.
    pub includes: Vec<IncludeSummary>,
    /// Parameter names and types.
    pub parameters: Vec<ParameterSummary>,
    /// Value set definitions.
    pub value_sets: Vec<ValueSetSummary>,
    /// Code system definitions.
    pub code_systems: Vec<CodeSystemSummary>,
    /// Expression definition summaries.
    pub expressions: Vec<DefinitionSummary>,
    /// Function definition summaries.
    pub functions: Vec<DefinitionSummary>,
    /// Retrieve nodes found in expression bodies.
    pub retrieves: Vec<RetrieveRequirement>,
    /// Expression node type counts.
    pub expression_node_counts: BTreeMap<String, usize>,
    /// Dependency graph for named expression/function definitions.
    pub dependencies: Vec<DefinitionDependencies>,
}

/// ELM using declaration summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsingSummary {
    /// Local model identifier.
    pub local_identifier: Option<String>,
    /// Model URI.
    pub uri: Option<String>,
    /// Model version.
    pub version: Option<String>,
}

/// ELM include declaration summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IncludeSummary {
    /// Local include alias.
    pub local_identifier: Option<String>,
    /// Included library path.
    pub path: Option<String>,
    /// Included library version.
    pub version: Option<String>,
}

/// Parameter summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSummary {
    /// Parameter name.
    pub name: Option<String>,
    /// ELM parameter type name.
    pub type_name: Option<String>,
}

/// Value set summary.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetSummary {
    /// Value set name.
    pub name: Option<String>,
    /// Value set canonical URL.
    pub id: Option<String>,
    /// Value set version.
    pub version: Option<String>,
}

/// Code system summary.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemSummary {
    /// Code system name.
    pub name: Option<String>,
    /// Code system canonical URL.
    pub id: Option<String>,
    /// Code system version.
    pub version: Option<String>,
}

/// Named definition summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionSummary {
    /// Definition name.
    pub name: Option<String>,
    /// Evaluation context.
    pub context: Option<String>,
    /// ELM result type name.
    pub result_type_name: Option<String>,
}

/// Named definition dependency summary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionDependencies {
    /// Definition name.
    pub name: String,
    /// Referenced expression definitions.
    pub expression_refs: Vec<String>,
    /// Referenced parameters.
    pub parameter_refs: Vec<String>,
    /// Referenced value sets.
    pub value_set_refs: Vec<String>,
    /// Referenced codes.
    pub code_refs: Vec<String>,
    /// Referenced functions.
    pub function_refs: Vec<String>,
}

/// Data requirements extracted from ELM.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirements {
    /// Library name.
    pub library: Option<String>,
    /// Library version.
    pub version: Option<String>,
    /// Unique FHIR resource types required by retrieve nodes.
    pub resources: Vec<String>,
    /// Value sets declared by the library.
    pub value_sets: Vec<ValueSetSummary>,
    /// Code systems declared by the library.
    pub code_systems: Vec<CodeSystemSummary>,
    /// Parameters declared by the library.
    pub parameters: Vec<ParameterSummary>,
    /// Retrieve nodes found in expression bodies.
    pub retrieves: Vec<RetrieveRequirement>,
}

/// Retrieve requirement extracted from ELM.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrieveRequirement {
    /// Definition containing the retrieve.
    pub definition: String,
    /// Retrieved model type.
    pub data_type: Option<String>,
    /// Normalized resource name.
    pub resource: Option<String>,
    /// Template/profile identifier.
    pub template_id: Option<String>,
    /// Code property used by the retrieve.
    pub code_property: Option<String>,
    /// Code comparator used by the retrieve.
    pub code_comparator: Option<String>,
    /// Date property used by the retrieve.
    pub date_property: Option<String>,
    /// Value sets referenced by the retrieve code expression.
    pub value_set_refs: Vec<String>,
}

/// Relational plan for a CQL library.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationalPlan {
    /// Library name.
    pub library: Option<String>,
    /// Target backend vocabulary.
    pub target: String,
    /// Planned named definitions.
    pub expressions: Vec<RelationalExpressionPlan>,
}

/// Relational plan for a named definition.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationalExpressionPlan {
    /// Definition name.
    pub name: String,
    /// Root relational node.
    pub root: RelNode,
}

/// A simple, serializable relational node.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelNode {
    /// Node operation.
    pub op: String,
    /// Node details.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub detail: BTreeMap<String, String>,
    /// Input nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<RelNode>,
}

/// Lowering support report for a target backend.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LowerCheckReport {
    /// Lowering target name.
    pub target: String,
    /// Whether all encountered node kinds are supported by the first-pass lowerer.
    pub supported: bool,
    /// Supported ELM node kinds encountered in the library.
    pub supported_nodes: Vec<NodeSupport>,
    /// Unsupported ELM node kinds encountered in the library.
    pub unsupported_nodes: Vec<NodeSupport>,
    /// Human-readable notes about the lowering boundary.
    pub notes: Vec<String>,
}

/// Count for a supported or unsupported ELM node kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSupport {
    /// ELM node kind.
    pub node_type: String,
    /// Encounter count.
    pub count: usize,
}

/// Inspect a compiled ELM library.
pub fn inspect_elm(library: &Library) -> ElmInspection {
    let mut expression_node_counts = BTreeMap::new();
    let mut retrieves = Vec::new();
    let mut dependencies = Vec::new();
    let mut expressions = Vec::new();
    let mut functions = Vec::new();

    for_each_definition(library, |name, summary, expression| {
        if summary.is_function {
            functions.push(DefinitionSummary {
                name: Some(name.clone()),
                context: summary.context,
                result_type_name: summary.result_type_name,
            });
        } else {
            expressions.push(DefinitionSummary {
                name: Some(name.clone()),
                context: summary.context,
                result_type_name: summary.result_type_name,
            });
        }

        if let Some(expr) = expression {
            let value = expression_to_value(expr);
            collect_node_counts(&value, &mut expression_node_counts);
            retrieves.extend(collect_retrieves(&name, &value));
            dependencies.push(collect_dependencies(&name, &value));
        } else {
            dependencies.push(DefinitionDependencies {
                name,
                expression_refs: Vec::new(),
                parameter_refs: Vec::new(),
                value_set_refs: Vec::new(),
                code_refs: Vec::new(),
                function_refs: Vec::new(),
            });
        }
    });

    ElmInspection {
        library: library_name(library),
        version: library_version(library),
        usings: using_summaries(library),
        includes: include_summaries(library),
        parameters: parameter_summaries(library),
        value_sets: value_set_summaries(library),
        code_systems: code_system_summaries(library),
        expressions,
        functions,
        retrieves,
        expression_node_counts,
        dependencies,
    }
}

/// Extract data requirements from a compiled ELM library.
pub fn data_requirements(library: &Library) -> DataRequirements {
    let inspection = inspect_elm(library);
    let resources = inspection
        .retrieves
        .iter()
        .filter_map(|r| r.resource.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    DataRequirements {
        library: inspection.library,
        version: inspection.version,
        resources,
        value_sets: inspection.value_sets,
        code_systems: inspection.code_systems,
        parameters: inspection.parameters,
        retrieves: inspection.retrieves,
    }
}

/// Build a first-pass relational plan from a compiled ELM library.
pub fn relational_plan(library: &Library, target: impl Into<String>) -> RelationalPlan {
    let mut expressions = Vec::new();
    for_each_definition(library, |name, _summary, expression| {
        if let Some(expr) = expression {
            expressions.push(RelationalExpressionPlan {
                name,
                root: plan_expression(&expression_to_value(expr)),
            });
        }
    });

    RelationalPlan {
        library: library_name(library),
        target: target.into(),
        expressions,
    }
}

/// Check whether a compiled ELM library can lower to a target.
pub fn lower_check(library: &Library, target: impl Into<String>) -> LowerCheckReport {
    let target = target.into();
    let inspection = inspect_elm(library);
    let mut supported_nodes = Vec::new();
    let mut unsupported_nodes = Vec::new();

    for (node_type, count) in inspection.expression_node_counts {
        let item = NodeSupport { node_type, count };
        if is_supported_for_first_pass(&item.node_type) {
            supported_nodes.push(item);
        } else {
            unsupported_nodes.push(item);
        }
    }

    let supported = unsupported_nodes.is_empty();
    let notes = vec![
        "This report covers the first-pass relational lowerer, not full CQL semantics."
            .to_string(),
        "Terminology expansion, complete interval precision, quantities, and complex list semantics may still require fallback evaluation.".to_string(),
    ];

    LowerCheckReport {
        target,
        supported,
        supported_nodes,
        unsupported_nodes,
        notes,
    }
}

/// Render a compact human-readable ELM inspection.
pub fn format_elm_inspection(inspection: &ElmInspection) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Library: {}\n",
        display_optional(inspection.library.as_deref())
    ));
    if let Some(version) = &inspection.version {
        out.push_str(&format!("Version: {version}\n"));
    }
    out.push_str(&format!("Usings: {}\n", inspection.usings.len()));
    out.push_str(&format!("Includes: {}\n", inspection.includes.len()));
    out.push_str(&format!("Parameters: {}\n", inspection.parameters.len()));
    out.push_str(&format!("Value sets: {}\n", inspection.value_sets.len()));
    out.push_str(&format!(
        "Code systems: {}\n",
        inspection.code_systems.len()
    ));
    out.push_str(&format!("Expressions: {}\n", inspection.expressions.len()));
    out.push_str(&format!("Functions: {}\n", inspection.functions.len()));
    out.push_str(&format!("Retrieves: {}\n", inspection.retrieves.len()));

    if !inspection.retrieves.is_empty() {
        out.push_str("\nRetrieves:\n");
        for retrieve in &inspection.retrieves {
            out.push_str(&format!(
                "  - {}: {}\n",
                retrieve.definition,
                display_optional(retrieve.resource.as_deref())
            ));
        }
    }

    if !inspection.expression_node_counts.is_empty() {
        out.push_str("\nELM node counts:\n");
        for (node, count) in &inspection.expression_node_counts {
            out.push_str(&format!("  - {node}: {count}\n"));
        }
    }

    out
}

/// Render a compact human-readable dependency graph.
pub fn format_dependencies(inspection: &ElmInspection) -> String {
    let mut out = String::new();
    for dep in &inspection.dependencies {
        out.push_str(&format!("{}\n", dep.name));
        format_list(&mut out, "expression refs", &dep.expression_refs);
        format_list(&mut out, "parameter refs", &dep.parameter_refs);
        format_list(&mut out, "value set refs", &dep.value_set_refs);
        format_list(&mut out, "code refs", &dep.code_refs);
        format_list(&mut out, "function refs", &dep.function_refs);
    }
    out
}

/// Render data requirements as text.
pub fn format_data_requirements(requirements: &DataRequirements) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Library: {}\n",
        display_optional(requirements.library.as_deref())
    ));
    format_list(&mut out, "resources", &requirements.resources);
    if !requirements.value_sets.is_empty() {
        out.push_str("value sets:\n");
        for vs in &requirements.value_sets {
            out.push_str(&format!(
                "  - {} ({})\n",
                display_optional(vs.name.as_deref()),
                display_optional(vs.id.as_deref())
            ));
        }
    }
    if !requirements.retrieves.is_empty() {
        out.push_str("retrieves:\n");
        for retrieve in &requirements.retrieves {
            out.push_str(&format!(
                "  - {}: resource={} codeProperty={} dateProperty={}\n",
                retrieve.definition,
                display_optional(retrieve.resource.as_deref()),
                display_optional(retrieve.code_property.as_deref()),
                display_optional(retrieve.date_property.as_deref())
            ));
        }
    }
    out
}

/// Render a relational plan as an indented tree.
pub fn format_relational_plan(plan: &RelationalPlan) -> String {
    let mut out = String::new();
    out.push_str(&format!("Target: {}\n", plan.target));
    for expr in &plan.expressions {
        out.push_str(&format!("\n{}\n", expr.name));
        format_node(&mut out, &expr.root, 1);
    }
    out
}

/// Render a lower-check report as text.
pub fn format_lower_check(report: &LowerCheckReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("Target: {}\n", report.target));
    out.push_str(&format!("Supported: {}\n", report.supported));
    if !report.supported_nodes.is_empty() {
        out.push_str("\nSupported nodes:\n");
        for node in &report.supported_nodes {
            out.push_str(&format!("  - {}: {}\n", node.node_type, node.count));
        }
    }
    if !report.unsupported_nodes.is_empty() {
        out.push_str("\nUnsupported nodes:\n");
        for node in &report.unsupported_nodes {
            out.push_str(&format!("  - {}: {}\n", node.node_type, node.count));
        }
    }
    if !report.notes.is_empty() {
        out.push_str("\nNotes:\n");
        for note in &report.notes {
            out.push_str(&format!("  - {note}\n"));
        }
    }
    out
}

#[derive(Debug)]
struct DefinitionMeta {
    context: Option<String>,
    result_type_name: Option<String>,
    is_function: bool,
}

fn for_each_definition(
    library: &Library,
    mut f: impl FnMut(String, DefinitionMeta, &Option<Box<Expression>>),
) {
    if let Some(statements) = &library.statements {
        for statement in &statements.defs {
            match statement {
                StatementDef::Expression(expr) => {
                    let name = expr
                        .name
                        .clone()
                        .unwrap_or_else(|| "<anonymous>".to_string());
                    f(
                        name,
                        DefinitionMeta {
                            context: expr.context.clone(),
                            result_type_name: expr.result_type_name.clone(),
                            is_function: false,
                        },
                        &expr.expression,
                    );
                }
                StatementDef::Function(func) => {
                    let name = func
                        .name
                        .clone()
                        .unwrap_or_else(|| "<anonymous>".to_string());
                    f(
                        name,
                        DefinitionMeta {
                            context: func.context.clone(),
                            result_type_name: func.result_type_name.clone(),
                            is_function: true,
                        },
                        &func.expression,
                    );
                }
            }
        }
    }
}

fn library_name(library: &Library) -> Option<String> {
    library.identifier.as_ref().and_then(|id| id.id.clone())
}

fn library_version(library: &Library) -> Option<String> {
    library
        .identifier
        .as_ref()
        .and_then(|id| id.version.clone())
}

fn using_summaries(library: &Library) -> Vec<UsingSummary> {
    library
        .usings
        .as_ref()
        .map(|defs| {
            defs.defs
                .iter()
                .map(|using| UsingSummary {
                    local_identifier: using.local_identifier.clone(),
                    uri: using.uri.clone(),
                    version: using.version.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn include_summaries(library: &Library) -> Vec<IncludeSummary> {
    library
        .includes
        .as_ref()
        .map(|defs| {
            defs.defs
                .iter()
                .map(|include| IncludeSummary {
                    local_identifier: include.local_identifier.clone(),
                    path: include.path.clone(),
                    version: include.version.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parameter_summaries(library: &Library) -> Vec<ParameterSummary> {
    library
        .parameters
        .as_ref()
        .map(|defs| {
            defs.defs
                .iter()
                .map(|param| ParameterSummary {
                    name: param.name.clone(),
                    type_name: param.parameter_type_name.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn value_set_summaries(library: &Library) -> Vec<ValueSetSummary> {
    library
        .value_sets
        .as_ref()
        .map(|defs| {
            defs.defs
                .iter()
                .map(|vs| ValueSetSummary {
                    name: vs.name.clone(),
                    id: vs.id.clone(),
                    version: vs.version.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn code_system_summaries(library: &Library) -> Vec<CodeSystemSummary> {
    library
        .code_systems
        .as_ref()
        .map(|defs| {
            defs.defs
                .iter()
                .map(|cs| CodeSystemSummary {
                    name: cs.name.clone(),
                    id: cs.id.clone(),
                    version: cs.version.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn expression_to_value(expression: &Expression) -> Value {
    serde_json::to_value(expression).unwrap_or(Value::Null)
}

fn collect_node_counts(value: &Value, counts: &mut BTreeMap<String, usize>) {
    match value {
        Value::Object(map) => {
            if let Some(node_type) = map.get("type").and_then(Value::as_str) {
                *counts.entry(node_type.to_string()).or_default() += 1;
            }
            for child in map.values() {
                collect_node_counts(child, counts);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_node_counts(item, counts);
            }
        }
        _ => {}
    }
}

fn collect_retrieves(definition: &str, value: &Value) -> Vec<RetrieveRequirement> {
    let mut retrieves = Vec::new();
    walk_objects(value, &mut |object| {
        if object.get("type").and_then(Value::as_str) == Some("Retrieve") {
            let data_type = string_field(object, "dataType");
            let code_expr = object.get("codes");
            retrieves.push(RetrieveRequirement {
                definition: definition.to_string(),
                resource: data_type.as_deref().map(normalize_resource_type),
                data_type,
                template_id: string_field(object, "templateId"),
                code_property: string_field(object, "codeProperty"),
                code_comparator: string_field(object, "codeComparator"),
                date_property: string_field(object, "dateProperty"),
                value_set_refs: code_expr.map(collect_named_refs).unwrap_or_default(),
            });
        }
    });
    retrieves
}

fn collect_dependencies(definition: &str, value: &Value) -> DefinitionDependencies {
    let mut expression_refs = BTreeSet::new();
    let mut parameter_refs = BTreeSet::new();
    let mut value_set_refs = BTreeSet::new();
    let mut code_refs = BTreeSet::new();
    let mut function_refs = BTreeSet::new();

    walk_objects(
        value,
        &mut |object| match object.get("type").and_then(Value::as_str) {
            Some("ExpressionRef") => insert_name(object, &mut expression_refs),
            Some("ParameterRef") => insert_name(object, &mut parameter_refs),
            Some("ValueSetRef") => insert_name(object, &mut value_set_refs),
            Some("CodeRef") => insert_name(object, &mut code_refs),
            Some("FunctionRef") => insert_name(object, &mut function_refs),
            _ => {}
        },
    );

    DefinitionDependencies {
        name: definition.to_string(),
        expression_refs: expression_refs.into_iter().collect(),
        parameter_refs: parameter_refs.into_iter().collect(),
        value_set_refs: value_set_refs.into_iter().collect(),
        code_refs: code_refs.into_iter().collect(),
        function_refs: function_refs.into_iter().collect(),
    }
}

fn collect_named_refs(value: &Value) -> Vec<String> {
    let mut refs = BTreeSet::new();
    walk_objects(value, &mut |object| {
        if matches!(
            object.get("type").and_then(Value::as_str),
            Some("ValueSetRef" | "CodeRef" | "CodeSystemRef" | "ConceptRef")
        ) {
            insert_name(object, &mut refs);
        }
    });
    refs.into_iter().collect()
}

fn insert_name(object: &serde_json::Map<String, Value>, names: &mut BTreeSet<String>) {
    if let Some(name) = object.get("name").and_then(Value::as_str) {
        names.insert(name.to_string());
    }
}

fn walk_objects(value: &Value, f: &mut impl FnMut(&serde_json::Map<String, Value>)) {
    match value {
        Value::Object(map) => {
            f(map);
            for child in map.values() {
                walk_objects(child, f);
            }
        }
        Value::Array(items) => {
            for item in items {
                walk_objects(item, f);
            }
        }
        _ => {}
    }
}

fn string_field(object: &serde_json::Map<String, Value>, field: &str) -> Option<String> {
    object.get(field).and_then(|value| match value {
        Value::String(s) => Some(s.clone()),
        Value::Object(map) => map
            .get("name")
            .or_else(|| map.get("id"))
            .and_then(Value::as_str)
            .map(ToString::to_string),
        _ => None,
    })
}

fn normalize_resource_type(data_type: &str) -> String {
    data_type
        .rsplit(['.', '}'])
        .next()
        .unwrap_or(data_type)
        .to_string()
}

fn plan_expression(value: &Value) -> RelNode {
    match value.get("type").and_then(Value::as_str) {
        Some("Retrieve") => {
            let data_type = value
                .as_object()
                .and_then(|object| string_field(object, "dataType"));
            node(
                "Scan",
                [
                    ("dataType", data_type.clone().unwrap_or_default()),
                    (
                        "resource",
                        data_type
                            .as_deref()
                            .map(normalize_resource_type)
                            .unwrap_or_default(),
                    ),
                ],
                Vec::new(),
            )
        }
        Some("Query") => plan_query(value),
        Some("Exists") => node(
            "Exists",
            [],
            value
                .get("operand")
                .map(|operand| vec![plan_expression(operand)])
                .unwrap_or_default(),
        ),
        Some(
            "And" | "Or" | "Not" | "InValueSet" | "AnyInValueSet" | "Equal" | "NotEqual" | "Less"
            | "LessOrEqual" | "Greater" | "GreaterOrEqual" | "Overlaps" | "IncludedIn" | "Includes"
            | "Before" | "After" | "SameOrBefore" | "SameOrAfter" | "Property" | "Literal"
            | "ValueSetRef" | "CodeRef" | "ExpressionRef" | "ParameterRef" | "AliasRef",
        ) => {
            let op = value
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("Predicate");
            node("Expr", [("kind", op.to_string())], Vec::new())
        }
        Some(other) => node("Unsupported", [("elmType", other.to_string())], Vec::new()),
        None => node(
            "Unsupported",
            [("elmType", "Unknown".to_string())],
            Vec::new(),
        ),
    }
}

fn plan_query(value: &Value) -> RelNode {
    let mut current = value
        .get("source")
        .and_then(Value::as_array)
        .and_then(|sources| sources.first())
        .and_then(|source| source.get("expression"))
        .map(plan_expression)
        .unwrap_or_else(|| node("EmptyScan", [], Vec::new()));

    if let Some(relationships) = value.get("relationship").and_then(Value::as_array) {
        for relationship in relationships {
            let relation_kind = relationship
                .get("type")
                .and_then(Value::as_str)
                .unwrap_or("with")
                .to_string();
            let relation_plan = relationship
                .get("expression")
                .map(plan_expression)
                .unwrap_or_else(|| node("EmptyScan", [], Vec::new()));
            current = node(
                if relation_kind.eq_ignore_ascii_case("without") {
                    "AntiJoin"
                } else {
                    "SemiJoin"
                },
                [("relationship", relation_kind)],
                vec![current, relation_plan],
            );
        }
    }

    if let Some(predicate) = value.get("where") {
        current = node("Filter", [], vec![current, plan_expression(predicate)]);
    }

    if let Some(return_expr) = value
        .get("return")
        .and_then(|return_clause| return_clause.get("expression"))
    {
        current = node("Project", [], vec![current, plan_expression(return_expr)]);
    }

    if value.get("aggregate").is_some() {
        current = node("Aggregate", [], vec![current]);
    }

    current
}

fn node(
    op: impl Into<String>,
    details: impl IntoIterator<Item = (&'static str, String)>,
    inputs: Vec<RelNode>,
) -> RelNode {
    RelNode {
        op: op.into(),
        detail: details
            .into_iter()
            .filter(|(_, value)| !value.is_empty())
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
        inputs,
    }
}

fn is_supported_for_first_pass(node_type: &str) -> bool {
    matches!(
        node_type,
        "Retrieve"
            | "Query"
            | "AliasRef"
            | "IdentifierRef"
            | "ExpressionRef"
            | "ParameterRef"
            | "ValueSetRef"
            | "CodeRef"
            | "CodeSystemRef"
            | "ConceptRef"
            | "Property"
            | "Literal"
            | "Null"
            | "And"
            | "Or"
            | "Not"
            | "Equal"
            | "Equivalent"
            | "NotEqual"
            | "Less"
            | "LessOrEqual"
            | "Greater"
            | "GreaterOrEqual"
            | "Exists"
            | "InValueSet"
            | "AnyInValueSet"
            | "Union"
            | "Intersect"
            | "Except"
            | "Count"
            | "Sum"
            | "Min"
            | "Max"
            | "Avg"
            | "Interval"
            | "Start"
            | "End"
            | "Overlaps"
            | "IncludedIn"
            | "Includes"
            | "Before"
            | "After"
            | "SameOrBefore"
            | "SameOrAfter"
    )
}

fn format_list(out: &mut String, label: &str, values: &[String]) {
    out.push_str(&format!("{label}:"));
    if values.is_empty() {
        out.push_str(" none\n");
    } else {
        out.push('\n');
        for value in values {
            out.push_str(&format!("  - {value}\n"));
        }
    }
}

fn format_node(out: &mut String, node: &RelNode, depth: usize) {
    let indent = "  ".repeat(depth);
    if node.detail.is_empty() {
        out.push_str(&format!("{indent}{}\n", node.op));
    } else {
        let details = node
            .detail
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!("{indent}{} {details}\n", node.op));
    }
    for input in &node.inputs {
        format_node(out, input, depth + 1);
    }
}

fn display_optional(value: Option<&str>) -> &str {
    value.unwrap_or("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile;

    const CQL: &str = r#"
library Example version '1.0.0'
using FHIR version '4.0.1'
valueset "Diabetes": 'http://example.org/ValueSet/diabetes'
parameter MeasurementPeriod Interval<DateTime>
context Patient
define "Diabetes Conditions":
  [Condition: "Diabetes"]
define "Has Diabetes":
  exists "Diabetes Conditions"
"#;

    fn library() -> Library {
        compile(CQL, None).expect("compile").library
    }

    #[test]
    fn data_requirements_find_retrieve_resource() {
        let requirements = data_requirements(&library());
        assert!(requirements.resources.contains(&"Condition".to_string()));
        assert_eq!(requirements.value_sets.len(), 1);
    }

    #[test]
    fn lower_check_reports_nodes() {
        let report = lower_check(&library(), "sql-on-fhir");
        assert!(report
            .supported_nodes
            .iter()
            .any(|n| n.node_type == "Retrieve"));
    }
}
