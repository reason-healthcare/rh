use crate::datatype::DataType;
use crate::library::LibraryIdentifier;
use std::collections::HashMap;

/// The kind of symbol defined in a CQL library.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Expression,
    Function,
    Parameter,
    CodeSystem,
    ValueSet,
    Code,
    Concept,
    QueryAlias,
    Let,
    Operand,
    Context,
    Model,
    Include,
}

/// A resolved symbol with its type.
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub result_type: Option<DataType>,
    pub library: Option<LibraryIdentifier>,
    pub is_public: bool,
}

impl Symbol {
    pub fn new(name: impl Into<String>, kind: SymbolKind) -> Self {
        Self {
            name: name.into(),
            kind,
            result_type: None,
            library: None,
            is_public: true,
        }
    }

    pub fn with_type(mut self, result_type: DataType) -> Self {
        self.result_type = Some(result_type);
        self
    }
}

/// Signature for a function overload.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: String,
    pub operand_types: Vec<DataType>,
    pub result_type: DataType,
    pub is_fluent: bool,
    pub is_external: bool,
    pub library: Option<LibraryIdentifier>,
}

impl FunctionSignature {
    pub fn new(name: impl Into<String>, result_type: DataType) -> Self {
        Self {
            name: name.into(),
            operand_types: Vec::new(),
            result_type,
            is_fluent: false,
            is_external: false,
            library: None,
        }
    }
}

/// A lexical scope containing symbols and function overloads.
#[derive(Debug, Default, Clone)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
    functions: HashMap<String, Vec<FunctionSignature>>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, symbol: Symbol) -> Option<Symbol> {
        self.symbols.insert(symbol.name.clone(), symbol)
    }

    pub fn define_function(&mut self, signature: FunctionSignature) {
        self.functions
            .entry(signature.name.clone())
            .or_default()
            .push(signature);
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn resolve_functions(&self, name: &str) -> Option<&[FunctionSignature]> {
        self.functions.get(name).map(|v| v.as_slice())
    }
}

/// Manages a stack of scopes.
#[derive(Debug, Default)]
pub struct ScopeManager {
    scopes: Vec<Scope>,
}

impl ScopeManager {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) -> Option<Scope> {
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None
        }
    }

    pub fn register_symbol(&mut self, symbol: Symbol) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define(symbol);
        }
    }

    pub fn register_function(&mut self, signature: FunctionSignature) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.define_function(signature);
        }
    }

    pub fn resolve_symbol_unqualified(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.resolve(name) {
                return Some(sym);
            }
        }
        None
    }

    pub fn resolve_functions_unqualified(&self, name: &str) -> Option<&[FunctionSignature]> {
        for scope in self.scopes.iter().rev() {
            if let Some(funcs) = scope.resolve_functions(name) {
                return Some(funcs);
            }
        }
        None
    }
}

impl ScopeManager {
    pub fn resolve_symbol_qualified(&self, library: &str, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.resolve(name) {
                if sym.library.as_ref().map(|l| l.name.as_str()) == Some(library) {
                    return Some(sym);
                }
            }
        }
        None
    }

    pub fn resolve_symbol(&self, library: Option<&str>, name: &str) -> Option<&Symbol> {
        if let Some(lib) = library {
            self.resolve_symbol_qualified(lib, name)
        } else {
            self.resolve_symbol_unqualified(name)
        }
    }
}

impl ScopeManager {
    pub fn push_query_scope(&mut self) {
        self.push_scope();
    }

    pub fn pop_query_scope(&mut self) {
        self.pop_scope();
    }

    pub fn register_query_alias(&mut self, alias: impl Into<String>, result_type: DataType) {
        let symbol = Symbol::new(alias, SymbolKind::QueryAlias).with_type(result_type);
        self.register_symbol(symbol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datatype::SystemType;

    #[test]
    fn test_scope_nesting_and_shadowing() {
        let mut mgr = ScopeManager::new();

        let sym_outer = Symbol::new("x", SymbolKind::Parameter)
            .with_type(DataType::System(SystemType::Integer));
        mgr.register_symbol(sym_outer);

        assert!(mgr.resolve_symbol_unqualified("x").is_some());

        mgr.push_scope();
        let sym_inner =
            Symbol::new("x", SymbolKind::Let).with_type(DataType::System(SystemType::String));
        mgr.register_symbol(sym_inner);

        let resolved = mgr.resolve_symbol_unqualified("x").unwrap();
        assert_eq!(resolved.kind, SymbolKind::Let);

        mgr.pop_scope();
        let resolved = mgr.resolve_symbol_unqualified("x").unwrap();
        assert_eq!(resolved.kind, SymbolKind::Parameter);
    }
}
