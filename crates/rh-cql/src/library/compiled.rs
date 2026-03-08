//! Compiled library wrapper with O(1) definition lookup.
//!
//! Provides [`CompiledLibrary`] wrapping an ELM [`Library`] with pre-built
//! indexes for efficient definition access, plus [`FunctionRef`] and
//! [`DefinitionRef`] reference types.

use std::collections::HashMap;

use crate::elm::{
    AccessModifier, CodeDef, CodeSystemDef, ConceptDef, ContextDef, ExpressionDef, FunctionDef,
    IncludeDef, Library, OperandDef, ParameterDef, StatementDef, UsingDef, ValueSetDef,
    VersionedIdentifier,
};

use super::identifiers::LibraryIdentifier;

// =============================================================================
// LibraryIndex — internal, private
// =============================================================================

/// Internal lookup index for O(1) definition access by name.
///
/// Built once at `CompiledLibrary` construction and kept alongside the
/// underlying ELM `Library`.  All `get_*` lookups on `CompiledLibrary` use
/// this index instead of scanning slices on every call.
#[derive(Debug, Clone, Default)]
struct LibraryIndex {
    /// Expression name → index into `library.statements.defs`.
    expressions: HashMap<String, usize>,
    /// Function name → indices into `library.statements.defs` (all overloads).
    functions: HashMap<String, Vec<usize>>,
    /// Parameter name → index into `library.parameters.defs`.
    parameters: HashMap<String, usize>,
    /// Using local-identifier → index into `library.usings.defs`.
    usings: HashMap<String, usize>,
    /// Include local-identifier → index into `library.includes.defs`.
    includes: HashMap<String, usize>,
    /// Code-system name → index into `library.code_systems.defs`.
    code_systems: HashMap<String, usize>,
    /// Value-set name → index into `library.value_sets.defs`.
    value_sets: HashMap<String, usize>,
    /// Code name → index into `library.codes.defs`.
    codes: HashMap<String, usize>,
    /// Concept name → index into `library.concepts.defs`.
    concepts: HashMap<String, usize>,
    /// Context name → index into `library.contexts.defs`.
    contexts: HashMap<String, usize>,
}

/// Build a `LibraryIndex` from an ELM `Library`.
fn build_index(library: &Library) -> LibraryIndex {
    let mut idx = LibraryIndex::default();

    if let Some(stmts) = &library.statements {
        for (i, def) in stmts.defs.iter().enumerate() {
            match def {
                StatementDef::Expression(e) => {
                    if let Some(name) = &e.name {
                        idx.expressions.insert(name.clone(), i);
                    }
                }
                StatementDef::Function(f) => {
                    if let Some(name) = &f.name {
                        idx.functions.entry(name.clone()).or_default().push(i);
                    }
                }
            }
        }
    }

    if let Some(params) = &library.parameters {
        for (i, p) in params.defs.iter().enumerate() {
            if let Some(name) = &p.name {
                idx.parameters.insert(name.clone(), i);
            }
        }
    }

    if let Some(usings) = &library.usings {
        for (i, u) in usings.defs.iter().enumerate() {
            if let Some(name) = &u.local_identifier {
                idx.usings.insert(name.clone(), i);
            }
        }
    }

    if let Some(includes) = &library.includes {
        for (i, inc) in includes.defs.iter().enumerate() {
            if let Some(name) = &inc.local_identifier {
                idx.includes.insert(name.clone(), i);
            }
        }
    }

    if let Some(css) = &library.code_systems {
        for (i, cs) in css.defs.iter().enumerate() {
            if let Some(name) = &cs.name {
                idx.code_systems.insert(name.clone(), i);
            }
        }
    }

    if let Some(vss) = &library.value_sets {
        for (i, vs) in vss.defs.iter().enumerate() {
            if let Some(name) = &vs.name {
                idx.value_sets.insert(name.clone(), i);
            }
        }
    }

    if let Some(codes) = &library.codes {
        for (i, code) in codes.defs.iter().enumerate() {
            if let Some(name) = &code.name {
                idx.codes.insert(name.clone(), i);
            }
        }
    }

    if let Some(concepts) = &library.concepts {
        for (i, concept) in concepts.defs.iter().enumerate() {
            if let Some(name) = &concept.name {
                idx.concepts.insert(name.clone(), i);
            }
        }
    }

    if let Some(contexts) = &library.contexts {
        for (i, ctx) in contexts.defs.iter().enumerate() {
            if let Some(name) = &ctx.name {
                idx.contexts.insert(name.clone(), i);
            }
        }
    }

    idx
}

// =============================================================================
// CompiledLibrary
// =============================================================================

/// A compiled CQL library with convenient lookup methods.
///
/// `CompiledLibrary` wraps an ELM [`Library`] and provides efficient access to
/// its definitions by name. It also tracks the source location and resolved
/// dependencies.
///
/// # Example
///
/// ```
/// use rh_cql::library::{CompiledLibrary, LibraryIdentifier};
/// use rh_cql::elm::Library;
///
/// // Create from an ELM library
/// let elm = Library::default();
/// let compiled = CompiledLibrary::new(elm);
///
/// // Look up expressions by name
/// if let Some(expr_def) = compiled.get_expression("InPopulation") {
///     println!("Found expression: {:?}", expr_def.name);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct CompiledLibrary {
    /// The underlying ELM library.
    library: Library,
    /// Source location (file path or URI) if known.
    source_location: Option<String>,
    /// Pre-built lookup indexes for O(1) definition access.
    index: LibraryIndex,
}

impl CompiledLibrary {
    /// Create a new compiled library from an ELM library.
    pub fn new(library: Library) -> Self {
        let index = build_index(&library);
        Self {
            library,
            source_location: None,
            index,
        }
    }

    /// Create a compiled library with source location metadata.
    pub fn with_source_location(library: Library, location: impl Into<String>) -> Self {
        let index = build_index(&library);
        Self {
            library,
            source_location: Some(location.into()),
            index,
        }
    }

    /// Get the underlying ELM library.
    pub fn library(&self) -> &Library {
        &self.library
    }

    /// Get the source location if known.
    pub fn source_location(&self) -> Option<&str> {
        self.source_location.as_deref()
    }

    /// Get the library identifier.
    pub fn identifier(&self) -> Option<&VersionedIdentifier> {
        self.library.identifier.as_ref()
    }

    /// Get the library name.
    pub fn name(&self) -> Option<&str> {
        self.library
            .identifier
            .as_ref()
            .and_then(|id| id.id.as_deref())
    }

    /// Get the library version.
    pub fn version(&self) -> Option<&str> {
        self.library
            .identifier
            .as_ref()
            .and_then(|id| id.version.as_deref())
    }

    /// Convert to a LibraryIdentifier.
    pub fn to_library_identifier(&self) -> LibraryIdentifier {
        LibraryIdentifier::new(self.name().unwrap_or("unknown"), self.version())
    }

    // =========================================================================
    // Using declarations
    // =========================================================================

    /// Get all using declarations.
    pub fn usings(&self) -> &[UsingDef] {
        self.library
            .usings
            .as_ref()
            .map(|u| u.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a using declaration by local identifier.
    pub fn get_using(&self, local_identifier: &str) -> Option<&UsingDef> {
        let idx = self.index.usings.get(local_identifier)?;
        self.library.usings.as_ref()?.defs.get(*idx)
    }

    // =========================================================================
    // Include declarations
    // =========================================================================

    /// Get all include declarations.
    pub fn includes(&self) -> &[IncludeDef] {
        self.library
            .includes
            .as_ref()
            .map(|i| i.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get an include declaration by local identifier.
    pub fn get_include(&self, local_identifier: &str) -> Option<&IncludeDef> {
        let idx = self.index.includes.get(local_identifier)?;
        self.library.includes.as_ref()?.defs.get(*idx)
    }

    /// Get the library identifiers for all includes.
    pub fn include_identifiers(&self) -> Vec<LibraryIdentifier> {
        self.includes()
            .iter()
            .filter_map(|inc| {
                inc.path
                    .as_ref()
                    .map(|path| LibraryIdentifier::new(path.clone(), inc.version.clone()))
            })
            .collect()
    }

    // =========================================================================
    // Parameter definitions
    // =========================================================================

    /// Get all parameter definitions.
    pub fn parameters(&self) -> &[ParameterDef] {
        self.library
            .parameters
            .as_ref()
            .map(|p| p.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a parameter definition by name.
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterDef> {
        let idx = self.index.parameters.get(name)?;
        self.library.parameters.as_ref()?.defs.get(*idx)
    }

    /// Get all public parameter definitions.
    pub fn public_parameters(&self) -> Vec<&ParameterDef> {
        self.parameters()
            .iter()
            .filter(|p| p.access_level != Some(AccessModifier::Private))
            .collect()
    }

    // =========================================================================
    // Code system definitions
    // =========================================================================

    /// Get all code system definitions.
    pub fn code_systems(&self) -> &[CodeSystemDef] {
        self.library
            .code_systems
            .as_ref()
            .map(|cs| cs.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a code system definition by name.
    pub fn get_code_system(&self, name: &str) -> Option<&CodeSystemDef> {
        let idx = self.index.code_systems.get(name)?;
        self.library.code_systems.as_ref()?.defs.get(*idx)
    }

    // =========================================================================
    // Value set definitions
    // =========================================================================

    /// Get all value set definitions.
    pub fn value_sets(&self) -> &[ValueSetDef] {
        self.library
            .value_sets
            .as_ref()
            .map(|vs| vs.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a value set definition by name.
    pub fn get_value_set(&self, name: &str) -> Option<&ValueSetDef> {
        let idx = self.index.value_sets.get(name)?;
        self.library.value_sets.as_ref()?.defs.get(*idx)
    }

    // =========================================================================
    // Code definitions
    // =========================================================================

    /// Get all code definitions.
    pub fn codes(&self) -> &[CodeDef] {
        self.library
            .codes
            .as_ref()
            .map(|c| c.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a code definition by name.
    pub fn get_code(&self, name: &str) -> Option<&CodeDef> {
        let idx = self.index.codes.get(name)?;
        self.library.codes.as_ref()?.defs.get(*idx)
    }

    // =========================================================================
    // Concept definitions
    // =========================================================================

    /// Get all concept definitions.
    pub fn concepts(&self) -> &[ConceptDef] {
        self.library
            .concepts
            .as_ref()
            .map(|c| c.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a concept definition by name.
    pub fn get_concept(&self, name: &str) -> Option<&ConceptDef> {
        let idx = self.index.concepts.get(name)?;
        self.library.concepts.as_ref()?.defs.get(*idx)
    }

    // =========================================================================
    // Context definitions
    // =========================================================================

    /// Get all context definitions.
    pub fn contexts(&self) -> &[ContextDef] {
        self.library
            .contexts
            .as_ref()
            .map(|c| c.defs.as_slice())
            .unwrap_or(&[])
    }

    /// Get a context definition by name.
    pub fn get_context(&self, name: &str) -> Option<&ContextDef> {
        let idx = self.index.contexts.get(name)?;
        self.library.contexts.as_ref()?.defs.get(*idx)
    }

    // =========================================================================
    // Expression definitions
    // =========================================================================

    /// Get all expression definitions (statements).
    pub fn expressions(&self) -> Vec<&ExpressionDef> {
        self.library
            .statements
            .as_ref()
            .map(|s| {
                s.defs
                    .iter()
                    .filter_map(|stmt| match stmt {
                        StatementDef::Expression(expr) => Some(expr),
                        StatementDef::Function(_) => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get an expression definition by name.
    pub fn get_expression(&self, name: &str) -> Option<&ExpressionDef> {
        let idx = self.index.expressions.get(name)?;
        if let StatementDef::Expression(e) = self.library.statements.as_ref()?.defs.get(*idx)? {
            Some(e)
        } else {
            None
        }
    }

    /// Get all public expression definitions.
    pub fn public_expressions(&self) -> Vec<&ExpressionDef> {
        self.expressions()
            .into_iter()
            .filter(|e| e.access_level != Some(AccessModifier::Private))
            .collect()
    }

    /// Get all expression definitions for a specific context.
    pub fn expressions_for_context(&self, context: &str) -> Vec<&ExpressionDef> {
        self.expressions()
            .into_iter()
            .filter(|e| e.context.as_deref() == Some(context))
            .collect()
    }

    // =========================================================================
    // Function definitions
    // =========================================================================

    /// Get all function definitions.
    ///
    /// Functions are stored in the statements section alongside expression
    /// definitions and are distinguished by the `StatementDef::Function` variant.
    pub fn functions(&self) -> Vec<FunctionRef<'_>> {
        let Some(stmts) = &self.library.statements else {
            return Vec::new();
        };
        stmts
            .defs
            .iter()
            .filter_map(|def| {
                if let StatementDef::Function(f) = def {
                    Some(FunctionRef {
                        name: f.name.as_deref()?,
                        operands: &f.operand,
                        def: Some(f),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get a function definition by name.
    ///
    /// If there are multiple overloads, returns the first match.
    /// Use `get_function_overloads` to retrieve all overloads.
    pub fn get_function(&self, name: &str) -> Option<FunctionRef<'_>> {
        let indices = self.index.functions.get(name)?;
        let first_idx = indices.first()?;
        if let StatementDef::Function(f) = self.library.statements.as_ref()?.defs.get(*first_idx)? {
            Some(FunctionRef {
                name: f.name.as_deref()?,
                operands: &f.operand,
                def: Some(f),
            })
        } else {
            None
        }
    }

    /// Get a function definition by name and operand types.
    ///
    /// Matches first by name, then by operand count; type names in
    /// `operand_types` are compared against `OperandDef::operand_type_name`.
    /// Pass an empty slice to match any single-named overload regardless of
    /// arity.
    pub fn get_function_by_signature(
        &self,
        name: &str,
        operand_types: &[&str],
    ) -> Option<FunctionRef<'_>> {
        let overloads = self.get_function_overloads(name);
        if operand_types.is_empty() {
            return overloads.into_iter().next();
        }
        overloads.into_iter().find(|fr| {
            if fr.operands.len() != operand_types.len() {
                return false;
            }
            fr.operands
                .iter()
                .zip(operand_types.iter())
                .all(|(op, expected)| op.operand_type_name.as_deref() == Some(expected))
        })
    }

    /// Get all functions with a given name (all overloads).
    pub fn get_function_overloads(&self, name: &str) -> Vec<FunctionRef<'_>> {
        let Some(indices) = self.index.functions.get(name) else {
            return Vec::new();
        };
        let Some(stmts) = &self.library.statements else {
            return Vec::new();
        };
        indices
            .iter()
            .filter_map(|idx| {
                if let StatementDef::Function(f) = stmts.defs.get(*idx)? {
                    Some(FunctionRef {
                        name: f.name.as_deref()?,
                        operands: &f.operand,
                        def: Some(f),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    // =========================================================================
    // Definition lookup (any type)
    // =========================================================================

    /// Look up any definition by name.
    ///
    /// Searches expressions, parameters, code systems, value sets, codes,
    /// concepts, and contexts.
    pub fn get_definition(&self, name: &str) -> Option<DefinitionRef<'_>> {
        // Check expressions first (most common)
        if let Some(expr) = self.get_expression(name) {
            return Some(DefinitionRef::Expression(expr));
        }
        if let Some(param) = self.get_parameter(name) {
            return Some(DefinitionRef::Parameter(param));
        }
        if let Some(cs) = self.get_code_system(name) {
            return Some(DefinitionRef::CodeSystem(cs));
        }
        if let Some(vs) = self.get_value_set(name) {
            return Some(DefinitionRef::ValueSet(vs));
        }
        if let Some(code) = self.get_code(name) {
            return Some(DefinitionRef::Code(code));
        }
        if let Some(concept) = self.get_concept(name) {
            return Some(DefinitionRef::Concept(concept));
        }
        if let Some(ctx) = self.get_context(name) {
            return Some(DefinitionRef::Context(ctx));
        }
        None
    }

    /// Check if a definition with the given name exists.
    pub fn has_definition(&self, name: &str) -> bool {
        self.get_definition(name).is_some()
    }

    /// Get all definition names in this library.
    pub fn definition_names(&self) -> Vec<&str> {
        let mut names = Vec::new();

        for expr in self.expressions() {
            if let Some(name) = expr.name.as_deref() {
                names.push(name);
            }
        }
        for param in self.parameters() {
            if let Some(name) = param.name.as_deref() {
                names.push(name);
            }
        }
        for cs in self.code_systems() {
            if let Some(name) = cs.name.as_deref() {
                names.push(name);
            }
        }
        for vs in self.value_sets() {
            if let Some(name) = vs.name.as_deref() {
                names.push(name);
            }
        }
        for code in self.codes() {
            if let Some(name) = code.name.as_deref() {
                names.push(name);
            }
        }
        for concept in self.concepts() {
            if let Some(name) = concept.name.as_deref() {
                names.push(name);
            }
        }

        names
    }
}

impl From<Library> for CompiledLibrary {
    fn from(library: Library) -> Self {
        Self::new(library)
    }
}

// =============================================================================
// Reference types
// =============================================================================

/// A reference to a function definition.
///
/// This is a placeholder type that will be expanded when function definitions
/// are properly parsed from ELM JSON.
#[derive(Debug, Clone)]
pub struct FunctionRef<'a> {
    /// Function name.
    pub name: &'a str,
    /// Function operands.
    pub operands: &'a [OperandDef],
    /// The underlying definition (if available).
    pub def: Option<&'a FunctionDef>,
}

/// A reference to any definition in a library.
#[derive(Debug, Clone)]
pub enum DefinitionRef<'a> {
    /// An expression definition.
    Expression(&'a ExpressionDef),
    /// A parameter definition.
    Parameter(&'a ParameterDef),
    /// A code system definition.
    CodeSystem(&'a CodeSystemDef),
    /// A value set definition.
    ValueSet(&'a ValueSetDef),
    /// A code definition.
    Code(&'a CodeDef),
    /// A concept definition.
    Concept(&'a ConceptDef),
    /// A context definition.
    Context(&'a ContextDef),
}

impl<'a> DefinitionRef<'a> {
    /// Get the name of this definition.
    pub fn name(&self) -> Option<&str> {
        match self {
            DefinitionRef::Expression(e) => e.name.as_deref(),
            DefinitionRef::Parameter(p) => p.name.as_deref(),
            DefinitionRef::CodeSystem(cs) => cs.name.as_deref(),
            DefinitionRef::ValueSet(vs) => vs.name.as_deref(),
            DefinitionRef::Code(c) => c.name.as_deref(),
            DefinitionRef::Concept(c) => c.name.as_deref(),
            DefinitionRef::Context(c) => c.name.as_deref(),
        }
    }

    /// Get the access level of this definition.
    pub fn access_level(&self) -> Option<AccessModifier> {
        match self {
            DefinitionRef::Expression(e) => e.access_level.clone(),
            DefinitionRef::Parameter(p) => p.access_level.clone(),
            DefinitionRef::CodeSystem(cs) => cs.access_level.clone(),
            DefinitionRef::ValueSet(vs) => vs.access_level.clone(),
            DefinitionRef::Code(c) => c.access_level.clone(),
            DefinitionRef::Concept(c) => c.access_level.clone(),
            DefinitionRef::Context(_) => None, // Contexts don't have access levels
        }
    }

    /// Check if this definition is public.
    pub fn is_public(&self) -> bool {
        self.access_level() != Some(AccessModifier::Private)
    }
}
