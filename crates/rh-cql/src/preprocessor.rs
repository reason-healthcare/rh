//! CQL Preprocessor for extracting library information from AST.
//!
//! The preprocessor is the first phase of semantic analysis. It extracts
//! metadata from a parsed CQL AST without performing type resolution,
//! preparing the library for full compilation.
//!
//! # Overview
//!
//! The preprocessor:
//! 1. Collects library metadata (name, version)
//! 2. Extracts model dependencies (using declarations)
//! 3. Identifies library dependencies (include declarations)
//! 4. Catalogs all definitions (expressions, functions, parameters, terminology)
//!
//! # Example
//!
//! ```
//! use rh_cql::parser::CqlParser;
//! use rh_cql::preprocessor::Preprocessor;
//!
//! let source = r#"
//!     library Example version '1.0'
//!     using FHIR version '4.0.1'
//!     include FHIRHelpers version '4.0.1'
//!     parameter MeasurementPeriod Interval<DateTime>
//!     context Patient
//!     define InPopulation: true
//! "#;
//!
//! let ast = CqlParser::new().parse(source).unwrap();
//! let info = Preprocessor::process(&ast);
//!
//! assert_eq!(info.name(), Some("Example"));
//! assert_eq!(info.version(), Some("1.0"));
//! assert_eq!(info.model_dependencies().len(), 1);
//! assert_eq!(info.library_dependencies().len(), 1);
//! assert!(info.has_definition("InPopulation"));
//! ```

use crate::library::LibraryIdentifier;
use crate::parser::ast::{
    self, AccessModifier, CodeDef, CodeSystemDef, ConceptDef, ExpressionDef, FunctionDef,
    IncludeDef, ParameterDef, Statement, UsingDef, ValueSetDef,
};

/// Information about a CQL library extracted during preprocessing.
///
/// This struct contains all metadata needed before full semantic analysis:
/// - Library identity (name, version)
/// - Model dependencies (FHIR, QDM, etc.)
/// - Library dependencies (included libraries)
/// - Definition catalog (names and basic info, no types resolved)
#[derive(Debug, Clone, Default)]
pub struct LibraryInfo {
    /// Library name.
    name: Option<String>,
    /// Library version.
    version: Option<String>,
    /// Model dependencies (using declarations).
    models: Vec<ModelDependency>,
    /// Library dependencies (include declarations).
    libraries: Vec<LibraryDependency>,
    /// Expression definitions.
    expressions: Vec<ExpressionInfo>,
    /// Function definitions.
    functions: Vec<FunctionInfo>,
    /// Parameter definitions.
    parameters: Vec<ParameterInfo>,
    /// Code system definitions.
    code_systems: Vec<CodeSystemInfo>,
    /// Value set definitions.
    value_sets: Vec<ValueSetInfo>,
    /// Code definitions.
    codes: Vec<CodeInfo>,
    /// Concept definitions.
    concepts: Vec<ConceptInfo>,
    /// Context definitions.
    contexts: Vec<String>,
}

impl LibraryInfo {
    /// Create a new empty LibraryInfo.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the library name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get the library version.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Get the library identifier.
    pub fn identifier(&self) -> Option<LibraryIdentifier> {
        self.name
            .as_ref()
            .map(|n| LibraryIdentifier::new(n.clone(), self.version.clone()))
    }

    /// Get model dependencies (using declarations).
    pub fn model_dependencies(&self) -> &[ModelDependency] {
        &self.models
    }

    /// Get library dependencies (include declarations).
    pub fn library_dependencies(&self) -> &[LibraryDependency] {
        &self.libraries
    }

    /// Get library dependency identifiers.
    pub fn library_dependency_ids(&self) -> Vec<LibraryIdentifier> {
        self.libraries.iter().map(|d| d.identifier()).collect()
    }

    /// Get expression definitions.
    pub fn expressions(&self) -> &[ExpressionInfo] {
        &self.expressions
    }

    /// Get function definitions.
    pub fn functions(&self) -> &[FunctionInfo] {
        &self.functions
    }

    /// Get parameter definitions.
    pub fn parameters(&self) -> &[ParameterInfo] {
        &self.parameters
    }

    /// Get code system definitions.
    pub fn code_systems(&self) -> &[CodeSystemInfo] {
        &self.code_systems
    }

    /// Get value set definitions.
    pub fn value_sets(&self) -> &[ValueSetInfo] {
        &self.value_sets
    }

    /// Get code definitions.
    pub fn codes(&self) -> &[CodeInfo] {
        &self.codes
    }

    /// Get concept definitions.
    pub fn concepts(&self) -> &[ConceptInfo] {
        &self.concepts
    }

    /// Get context definitions.
    pub fn contexts(&self) -> &[String] {
        &self.contexts
    }

    /// Check if a definition with the given name exists.
    pub fn has_definition(&self, name: &str) -> bool {
        self.get_definition_kind(name).is_some()
    }

    /// Get the kind of definition for a name.
    pub fn get_definition_kind(&self, name: &str) -> Option<DefinitionKind> {
        if self.expressions.iter().any(|e| e.name == name) {
            return Some(DefinitionKind::Expression);
        }
        if self.functions.iter().any(|f| f.name == name) {
            return Some(DefinitionKind::Function);
        }
        if self.parameters.iter().any(|p| p.name == name) {
            return Some(DefinitionKind::Parameter);
        }
        if self.code_systems.iter().any(|cs| cs.name == name) {
            return Some(DefinitionKind::CodeSystem);
        }
        if self.value_sets.iter().any(|vs| vs.name == name) {
            return Some(DefinitionKind::ValueSet);
        }
        if self.codes.iter().any(|c| c.name == name) {
            return Some(DefinitionKind::Code);
        }
        if self.concepts.iter().any(|c| c.name == name) {
            return Some(DefinitionKind::Concept);
        }
        None
    }

    /// Get all definition names.
    pub fn definition_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        names.extend(self.expressions.iter().map(|e| e.name.as_str()));
        names.extend(self.functions.iter().map(|f| f.name.as_str()));
        names.extend(self.parameters.iter().map(|p| p.name.as_str()));
        names.extend(self.code_systems.iter().map(|cs| cs.name.as_str()));
        names.extend(self.value_sets.iter().map(|vs| vs.name.as_str()));
        names.extend(self.codes.iter().map(|c| c.name.as_str()));
        names.extend(self.concepts.iter().map(|c| c.name.as_str()));
        names
    }

    /// Get public definition names only.
    pub fn public_definition_names(&self) -> Vec<&str> {
        let mut names = Vec::new();
        names.extend(
            self.expressions
                .iter()
                .filter(|e| e.is_public)
                .map(|e| e.name.as_str()),
        );
        names.extend(
            self.functions
                .iter()
                .filter(|f| f.is_public)
                .map(|f| f.name.as_str()),
        );
        names.extend(
            self.parameters
                .iter()
                .filter(|p| p.is_public)
                .map(|p| p.name.as_str()),
        );
        names.extend(
            self.code_systems
                .iter()
                .filter(|cs| cs.is_public)
                .map(|cs| cs.name.as_str()),
        );
        names.extend(
            self.value_sets
                .iter()
                .filter(|vs| vs.is_public)
                .map(|vs| vs.name.as_str()),
        );
        names.extend(
            self.codes
                .iter()
                .filter(|c| c.is_public)
                .map(|c| c.name.as_str()),
        );
        names.extend(
            self.concepts
                .iter()
                .filter(|c| c.is_public)
                .map(|c| c.name.as_str()),
        );
        names
    }

    /// Check if a model is used.
    pub fn uses_model(&self, model_name: &str) -> bool {
        self.models.iter().any(|m| m.name == model_name)
    }

    /// Get a model dependency by name.
    pub fn get_model(&self, name: &str) -> Option<&ModelDependency> {
        self.models.iter().find(|m| m.name == name)
    }

    /// Check if a library is included.
    pub fn includes_library(&self, library_name: &str) -> bool {
        self.libraries.iter().any(|l| l.path == library_name)
    }

    /// Get a library dependency by alias or path.
    pub fn get_library(&self, alias_or_path: &str) -> Option<&LibraryDependency> {
        self.libraries
            .iter()
            .find(|l| l.alias.as_deref() == Some(alias_or_path) || l.path == alias_or_path)
    }

    /// Get an expression by name.
    pub fn get_expression(&self, name: &str) -> Option<&ExpressionInfo> {
        self.expressions.iter().find(|e| e.name == name)
    }

    /// Get a function by name.
    pub fn get_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get all functions with a given name (for overloads).
    pub fn get_functions(&self, name: &str) -> Vec<&FunctionInfo> {
        self.functions.iter().filter(|f| f.name == name).collect()
    }

    /// Get a parameter by name.
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterInfo> {
        self.parameters.iter().find(|p| p.name == name)
    }
}

/// Kind of definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionKind {
    Expression,
    Function,
    Parameter,
    CodeSystem,
    ValueSet,
    Code,
    Concept,
}

impl std::fmt::Display for DefinitionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefinitionKind::Expression => write!(f, "expression"),
            DefinitionKind::Function => write!(f, "function"),
            DefinitionKind::Parameter => write!(f, "parameter"),
            DefinitionKind::CodeSystem => write!(f, "codesystem"),
            DefinitionKind::ValueSet => write!(f, "valueset"),
            DefinitionKind::Code => write!(f, "code"),
            DefinitionKind::Concept => write!(f, "concept"),
        }
    }
}

/// Model dependency from a using declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDependency {
    /// Model name (e.g., "FHIR", "QDM").
    pub name: String,
    /// Model version.
    pub version: Option<String>,
}

impl ModelDependency {
    /// Create a new model dependency.
    pub fn new(name: impl Into<String>, version: Option<impl Into<String>>) -> Self {
        Self {
            name: name.into(),
            version: version.map(|v| v.into()),
        }
    }
}

/// Library dependency from an include declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryDependency {
    /// Library path.
    pub path: String,
    /// Library version.
    pub version: Option<String>,
    /// Local alias.
    pub alias: Option<String>,
}

impl LibraryDependency {
    /// Create a new library dependency.
    pub fn new(
        path: impl Into<String>,
        version: Option<impl Into<String>>,
        alias: Option<impl Into<String>>,
    ) -> Self {
        Self {
            path: path.into(),
            version: version.map(|v| v.into()),
            alias: alias.map(|a| a.into()),
        }
    }

    /// Get the library identifier.
    pub fn identifier(&self) -> LibraryIdentifier {
        LibraryIdentifier::new(&self.path, self.version.as_deref())
    }

    /// Get the local name (alias or path).
    pub fn local_name(&self) -> &str {
        self.alias.as_deref().unwrap_or(&self.path)
    }
}

/// Expression definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionInfo {
    /// Expression name.
    pub name: String,
    /// Whether the expression is public.
    pub is_public: bool,
}

/// Function definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionInfo {
    /// Function name.
    pub name: String,
    /// Number of parameters.
    pub arity: usize,
    /// Parameter names.
    pub parameter_names: Vec<String>,
    /// Whether the function is public.
    pub is_public: bool,
    /// Whether the function is fluent.
    pub is_fluent: bool,
    /// Whether the function is external.
    pub is_external: bool,
}

/// Parameter definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterInfo {
    /// Parameter name.
    pub name: String,
    /// Whether the parameter is public.
    pub is_public: bool,
    /// Whether the parameter has a default value.
    pub has_default: bool,
}

/// Code system definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeSystemInfo {
    /// Code system name.
    pub name: String,
    /// Code system ID (URI/OID).
    pub id: String,
    /// Code system version.
    pub version: Option<String>,
    /// Whether the code system is public.
    pub is_public: bool,
}

/// Value set definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueSetInfo {
    /// Value set name.
    pub name: String,
    /// Value set ID (URI/OID).
    pub id: String,
    /// Value set version.
    pub version: Option<String>,
    /// Whether the value set is public.
    pub is_public: bool,
}

/// Code definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeInfo {
    /// Code name.
    pub name: String,
    /// Code value.
    pub code: String,
    /// Code system reference.
    pub codesystem: String,
    /// Display text.
    pub display: Option<String>,
    /// Whether the code is public.
    pub is_public: bool,
}

/// Concept definition info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptInfo {
    /// Concept name.
    pub name: String,
    /// Code references.
    pub codes: Vec<String>,
    /// Display text.
    pub display: Option<String>,
    /// Whether the concept is public.
    pub is_public: bool,
}

/// Preprocessor for extracting library information from AST.
///
/// The preprocessor performs a single pass over the AST to collect
/// all metadata needed for semantic analysis without resolving types.
pub struct Preprocessor;

impl Preprocessor {
    /// Process an AST library and extract library information.
    pub fn process(library: &ast::Library) -> LibraryInfo {
        let mut info = LibraryInfo::new();

        // Extract library identifier
        if let Some(id) = &library.identifier {
            info.name = Some(id.name.clone());
            info.version = id.version.clone();
        }

        // Extract model dependencies
        for using in &library.usings {
            info.models.push(Self::extract_model(using));
        }

        // Extract library dependencies
        for include in &library.includes {
            info.libraries.push(Self::extract_library(include));
        }

        // Extract code systems
        for cs in &library.codesystems {
            info.code_systems.push(Self::extract_code_system(cs));
        }

        // Extract value sets
        for vs in &library.valuesets {
            info.value_sets.push(Self::extract_value_set(vs));
        }

        // Extract codes
        for code in &library.codes {
            info.codes.push(Self::extract_code(code));
        }

        // Extract concepts
        for concept in &library.concepts {
            info.concepts.push(Self::extract_concept(concept));
        }

        // Extract parameters
        for param in &library.parameters {
            info.parameters.push(Self::extract_parameter(param));
        }

        // Extract contexts
        for ctx in &library.contexts {
            info.contexts.push(ctx.name.clone());
        }

        // Extract statements (expressions and functions)
        for stmt in &library.statements {
            match stmt {
                Statement::ExpressionDef(expr) => {
                    info.expressions.push(Self::extract_expression(expr));
                }
                Statement::FunctionDef(func) => {
                    info.functions.push(Self::extract_function(func));
                }
            }
        }

        info
    }

    fn extract_model(using: &UsingDef) -> ModelDependency {
        ModelDependency::new(&using.model_name, using.version.as_deref())
    }

    fn extract_library(include: &IncludeDef) -> LibraryDependency {
        LibraryDependency::new(
            &include.path,
            include.version.as_deref(),
            include.alias.as_deref(),
        )
    }

    fn extract_code_system(cs: &CodeSystemDef) -> CodeSystemInfo {
        CodeSystemInfo {
            name: cs.name.clone(),
            id: cs.id.clone(),
            version: cs.version.clone(),
            is_public: cs.access == AccessModifier::Public,
        }
    }

    fn extract_value_set(vs: &ValueSetDef) -> ValueSetInfo {
        ValueSetInfo {
            name: vs.name.clone(),
            id: vs.id.clone(),
            version: vs.version.clone(),
            is_public: vs.access == AccessModifier::Public,
        }
    }

    fn extract_code(code: &CodeDef) -> CodeInfo {
        CodeInfo {
            name: code.name.clone(),
            code: code.code.clone(),
            codesystem: code.codesystem.clone(),
            display: code.display.clone(),
            is_public: code.access == AccessModifier::Public,
        }
    }

    fn extract_concept(concept: &ConceptDef) -> ConceptInfo {
        ConceptInfo {
            name: concept.name.clone(),
            codes: concept.codes.clone(),
            display: concept.display.clone(),
            is_public: concept.access == AccessModifier::Public,
        }
    }

    fn extract_parameter(param: &ParameterDef) -> ParameterInfo {
        ParameterInfo {
            name: param.name.clone(),
            is_public: param.access == AccessModifier::Public,
            has_default: param.default.is_some(),
        }
    }

    fn extract_expression(expr: &ExpressionDef) -> ExpressionInfo {
        ExpressionInfo {
            name: expr.name.clone(),
            is_public: expr.access == AccessModifier::Public,
        }
    }

    fn extract_function(func: &FunctionDef) -> FunctionInfo {
        FunctionInfo {
            name: func.name.clone(),
            arity: func.parameters.len(),
            parameter_names: func.parameters.iter().map(|p| p.name.clone()).collect(),
            is_public: func.access == AccessModifier::Public,
            is_fluent: func.fluent,
            is_external: func.external,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{
        ContextDef, Expression, FunctionParameter, NamedTypeSpecifier, TypeSpecifier,
    };

    fn create_test_library() -> ast::Library {
        ast::Library {
            identifier: Some(ast::LibraryIdentifier {
                name: "TestLibrary".to_string(),
                version: Some("1.0.0".to_string()),
            }),
            usings: vec![
                UsingDef {
                    model_name: "FHIR".to_string(),
                    version: Some("4.0.1".to_string()),
                    location: None,
                },
                UsingDef {
                    model_name: "System".to_string(),
                    version: None,
                    location: None,
                },
            ],
            includes: vec![
                IncludeDef {
                    path: "FHIRHelpers".to_string(),
                    version: Some("4.0.1".to_string()),
                    alias: None,
                    location: None,
                },
                IncludeDef {
                    path: "CommonLogic".to_string(),
                    version: Some("1.0".to_string()),
                    alias: Some("Common".to_string()),
                    location: None,
                },
            ],
            codesystems: vec![CodeSystemDef {
                name: "LOINC".to_string(),
                id: "http://loinc.org".to_string(),
                version: Some("2.73".to_string()),
                access: AccessModifier::Public,
                location: None,
            }],
            valuesets: vec![ValueSetDef {
                name: "DiabetesCodes".to_string(),
                id: "http://example.org/vs/diabetes".to_string(),
                version: None,
                codesystems: vec![],
                access: AccessModifier::Public,
                location: None,
            }],
            codes: vec![CodeDef {
                name: "HbA1c".to_string(),
                code: "4548-4".to_string(),
                codesystem: "LOINC".to_string(),
                display: Some("Hemoglobin A1c".to_string()),
                access: AccessModifier::Public,
                location: None,
            }],
            concepts: vec![ConceptDef {
                name: "DiabetesMarkers".to_string(),
                codes: vec!["HbA1c".to_string()],
                display: Some("Diabetes Markers".to_string()),
                access: AccessModifier::Public,
                location: None,
            }],
            parameters: vec![
                ParameterDef {
                    name: "MeasurementPeriod".to_string(),
                    type_specifier: None,
                    default: None,
                    access: AccessModifier::Public,
                    location: None,
                },
                ParameterDef {
                    name: "InternalParam".to_string(),
                    type_specifier: None,
                    default: Some(Expression::Literal(crate::parser::ast::Literal::Boolean(
                        true,
                    ))),
                    access: AccessModifier::Private,
                    location: None,
                },
            ],
            contexts: vec![ContextDef {
                name: "Patient".to_string(),
                location: None,
            }],
            statements: vec![
                Statement::ExpressionDef(ExpressionDef {
                    name: "InPopulation".to_string(),
                    expression: Expression::Literal(crate::parser::ast::Literal::Boolean(true)),
                    access: AccessModifier::Public,
                    location: None,
                }),
                Statement::ExpressionDef(ExpressionDef {
                    name: "PrivateHelper".to_string(),
                    expression: Expression::Literal(crate::parser::ast::Literal::Boolean(false)),
                    access: AccessModifier::Private,
                    location: None,
                }),
                Statement::FunctionDef(FunctionDef {
                    name: "IsAdult".to_string(),
                    parameters: vec![FunctionParameter {
                        name: "birthDate".to_string(),
                        type_specifier: Some(TypeSpecifier::Named(NamedTypeSpecifier {
                            namespace: None,
                            name: "Date".to_string(),
                        })),
                    }],
                    return_type: Some(TypeSpecifier::Named(NamedTypeSpecifier {
                        namespace: None,
                        name: "Boolean".to_string(),
                    })),
                    body: Some(Expression::Literal(crate::parser::ast::Literal::Boolean(
                        true,
                    ))),
                    fluent: false,
                    external: false,
                    access: AccessModifier::Public,
                    location: None,
                }),
                Statement::FunctionDef(FunctionDef {
                    name: "ToAge".to_string(),
                    parameters: vec![FunctionParameter {
                        name: "value".to_string(),
                        type_specifier: None,
                    }],
                    return_type: None,
                    body: None,
                    fluent: true,
                    external: true,
                    access: AccessModifier::Public,
                    location: None,
                }),
            ],
        }
    }

    #[test]
    fn test_preprocessor_library_identity() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        assert_eq!(info.name(), Some("TestLibrary"));
        assert_eq!(info.version(), Some("1.0.0"));

        let id = info.identifier().unwrap();
        assert_eq!(id.name, "TestLibrary");
        assert_eq!(id.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_preprocessor_empty_library() {
        let library = ast::Library::default();
        let info = Preprocessor::process(&library);

        assert!(info.name().is_none());
        assert!(info.version().is_none());
        assert!(info.identifier().is_none());
        assert!(info.model_dependencies().is_empty());
        assert!(info.library_dependencies().is_empty());
    }

    #[test]
    fn test_preprocessor_model_dependencies() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let models = info.model_dependencies();
        assert_eq!(models.len(), 2);

        assert!(info.uses_model("FHIR"));
        assert!(info.uses_model("System"));
        assert!(!info.uses_model("QDM"));

        let fhir = info.get_model("FHIR").unwrap();
        assert_eq!(fhir.name, "FHIR");
        assert_eq!(fhir.version, Some("4.0.1".to_string()));

        let system = info.get_model("System").unwrap();
        assert_eq!(system.version, None);
    }

    #[test]
    fn test_preprocessor_library_dependencies() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let libs = info.library_dependencies();
        assert_eq!(libs.len(), 2);

        assert!(info.includes_library("FHIRHelpers"));
        assert!(info.includes_library("CommonLogic"));
        assert!(!info.includes_library("NonExistent"));

        let helpers = info.get_library("FHIRHelpers").unwrap();
        assert_eq!(helpers.path, "FHIRHelpers");
        assert_eq!(helpers.version, Some("4.0.1".to_string()));
        assert!(helpers.alias.is_none());
        assert_eq!(helpers.local_name(), "FHIRHelpers");

        // Look up by alias
        let common = info.get_library("Common").unwrap();
        assert_eq!(common.path, "CommonLogic");
        assert_eq!(common.alias, Some("Common".to_string()));
        assert_eq!(common.local_name(), "Common");
    }

    #[test]
    fn test_preprocessor_library_dependency_ids() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let ids = info.library_dependency_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.iter().any(|id| id.name == "FHIRHelpers"));
        assert!(ids.iter().any(|id| id.name == "CommonLogic"));
    }

    #[test]
    fn test_preprocessor_expressions() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let exprs = info.expressions();
        assert_eq!(exprs.len(), 2);

        let in_pop = info.get_expression("InPopulation").unwrap();
        assert!(in_pop.is_public);

        let helper = info.get_expression("PrivateHelper").unwrap();
        assert!(!helper.is_public);

        assert!(info.get_expression("NonExistent").is_none());
    }

    #[test]
    fn test_preprocessor_functions() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let funcs = info.functions();
        assert_eq!(funcs.len(), 2);

        let is_adult = info.get_function("IsAdult").unwrap();
        assert_eq!(is_adult.arity, 1);
        assert_eq!(is_adult.parameter_names, vec!["birthDate"]);
        assert!(is_adult.is_public);
        assert!(!is_adult.is_fluent);
        assert!(!is_adult.is_external);

        let to_age = info.get_function("ToAge").unwrap();
        assert!(to_age.is_fluent);
        assert!(to_age.is_external);
    }

    #[test]
    fn test_preprocessor_parameters() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let params = info.parameters();
        assert_eq!(params.len(), 2);

        let mp = info.get_parameter("MeasurementPeriod").unwrap();
        assert!(mp.is_public);
        assert!(!mp.has_default);

        let internal = info.get_parameter("InternalParam").unwrap();
        assert!(!internal.is_public);
        assert!(internal.has_default);
    }

    #[test]
    fn test_preprocessor_terminology() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        // Code systems
        let code_systems = info.code_systems();
        assert_eq!(code_systems.len(), 1);
        assert_eq!(code_systems[0].name, "LOINC");
        assert_eq!(code_systems[0].id, "http://loinc.org");

        // Value sets
        let value_sets = info.value_sets();
        assert_eq!(value_sets.len(), 1);
        assert_eq!(value_sets[0].name, "DiabetesCodes");

        // Codes
        let codes = info.codes();
        assert_eq!(codes.len(), 1);
        assert_eq!(codes[0].name, "HbA1c");
        assert_eq!(codes[0].code, "4548-4");
        assert_eq!(codes[0].codesystem, "LOINC");

        // Concepts
        let concepts = info.concepts();
        assert_eq!(concepts.len(), 1);
        assert_eq!(concepts[0].name, "DiabetesMarkers");
        assert_eq!(concepts[0].codes, vec!["HbA1c"]);
    }

    #[test]
    fn test_preprocessor_contexts() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let contexts = info.contexts();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0], "Patient");
    }

    #[test]
    fn test_preprocessor_has_definition() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        assert!(info.has_definition("InPopulation"));
        assert!(info.has_definition("IsAdult"));
        assert!(info.has_definition("MeasurementPeriod"));
        assert!(info.has_definition("LOINC"));
        assert!(info.has_definition("DiabetesCodes"));
        assert!(info.has_definition("HbA1c"));
        assert!(info.has_definition("DiabetesMarkers"));
        assert!(!info.has_definition("NonExistent"));
    }

    #[test]
    fn test_preprocessor_definition_kind() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        assert_eq!(
            info.get_definition_kind("InPopulation"),
            Some(DefinitionKind::Expression)
        );
        assert_eq!(
            info.get_definition_kind("IsAdult"),
            Some(DefinitionKind::Function)
        );
        assert_eq!(
            info.get_definition_kind("MeasurementPeriod"),
            Some(DefinitionKind::Parameter)
        );
        assert_eq!(
            info.get_definition_kind("LOINC"),
            Some(DefinitionKind::CodeSystem)
        );
        assert_eq!(
            info.get_definition_kind("DiabetesCodes"),
            Some(DefinitionKind::ValueSet)
        );
        assert_eq!(
            info.get_definition_kind("HbA1c"),
            Some(DefinitionKind::Code)
        );
        assert_eq!(
            info.get_definition_kind("DiabetesMarkers"),
            Some(DefinitionKind::Concept)
        );
        assert!(info.get_definition_kind("NonExistent").is_none());
    }

    #[test]
    fn test_preprocessor_definition_names() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let names = info.definition_names();
        assert!(names.contains(&"InPopulation"));
        assert!(names.contains(&"PrivateHelper"));
        assert!(names.contains(&"IsAdult"));
        assert!(names.contains(&"MeasurementPeriod"));
        assert!(names.contains(&"LOINC"));
        assert!(names.contains(&"DiabetesCodes"));
        assert!(names.contains(&"HbA1c"));
        assert!(names.contains(&"DiabetesMarkers"));
    }

    #[test]
    fn test_preprocessor_public_definition_names() {
        let library = create_test_library();
        let info = Preprocessor::process(&library);

        let public_names = info.public_definition_names();
        assert!(public_names.contains(&"InPopulation"));
        assert!(!public_names.contains(&"PrivateHelper"));
        assert!(public_names.contains(&"IsAdult"));
        assert!(public_names.contains(&"MeasurementPeriod"));
        assert!(!public_names.contains(&"InternalParam"));
    }

    #[test]
    fn test_definition_kind_display() {
        assert_eq!(DefinitionKind::Expression.to_string(), "expression");
        assert_eq!(DefinitionKind::Function.to_string(), "function");
        assert_eq!(DefinitionKind::Parameter.to_string(), "parameter");
        assert_eq!(DefinitionKind::CodeSystem.to_string(), "codesystem");
        assert_eq!(DefinitionKind::ValueSet.to_string(), "valueset");
        assert_eq!(DefinitionKind::Code.to_string(), "code");
        assert_eq!(DefinitionKind::Concept.to_string(), "concept");
    }

    #[test]
    fn test_model_dependency_new() {
        let dep = ModelDependency::new("FHIR", Some("4.0.1"));
        assert_eq!(dep.name, "FHIR");
        assert_eq!(dep.version, Some("4.0.1".to_string()));

        let dep2 = ModelDependency::new("System", None::<String>);
        assert_eq!(dep2.name, "System");
        assert!(dep2.version.is_none());
    }

    #[test]
    fn test_library_dependency_new() {
        let dep = LibraryDependency::new("FHIRHelpers", Some("4.0.1"), None::<String>);
        assert_eq!(dep.path, "FHIRHelpers");
        assert_eq!(dep.version, Some("4.0.1".to_string()));
        assert!(dep.alias.is_none());
        assert_eq!(dep.local_name(), "FHIRHelpers");

        let dep2 = LibraryDependency::new("CommonLogic", Some("1.0"), Some("Common"));
        assert_eq!(dep2.path, "CommonLogic");
        assert_eq!(dep2.alias, Some("Common".to_string()));
        assert_eq!(dep2.local_name(), "Common");
    }

    #[test]
    fn test_library_dependency_identifier() {
        let dep = LibraryDependency::new("FHIRHelpers", Some("4.0.1"), None::<String>);
        let id = dep.identifier();
        assert_eq!(id.name, "FHIRHelpers");
        assert_eq!(id.version, Some("4.0.1".to_string()));
    }

    #[test]
    fn test_library_info_new() {
        let info = LibraryInfo::new();
        assert!(info.name().is_none());
        assert!(info.model_dependencies().is_empty());
    }

    #[test]
    fn test_get_functions_overloads() {
        let library = ast::Library {
            statements: vec![
                Statement::FunctionDef(FunctionDef {
                    name: "Add".to_string(),
                    parameters: vec![
                        FunctionParameter {
                            name: "a".to_string(),
                            type_specifier: None,
                        },
                        FunctionParameter {
                            name: "b".to_string(),
                            type_specifier: None,
                        },
                    ],
                    return_type: None,
                    body: None,
                    fluent: false,
                    external: false,
                    access: AccessModifier::Public,
                    location: None,
                }),
                Statement::FunctionDef(FunctionDef {
                    name: "Add".to_string(),
                    parameters: vec![
                        FunctionParameter {
                            name: "a".to_string(),
                            type_specifier: None,
                        },
                        FunctionParameter {
                            name: "b".to_string(),
                            type_specifier: None,
                        },
                        FunctionParameter {
                            name: "c".to_string(),
                            type_specifier: None,
                        },
                    ],
                    return_type: None,
                    body: None,
                    fluent: false,
                    external: false,
                    access: AccessModifier::Public,
                    location: None,
                }),
            ],
            ..Default::default()
        };

        let info = Preprocessor::process(&library);
        let add_funcs = info.get_functions("Add");
        assert_eq!(add_funcs.len(), 2);
        assert!(add_funcs.iter().any(|f| f.arity == 2));
        assert!(add_funcs.iter().any(|f| f.arity == 3));
    }
}
