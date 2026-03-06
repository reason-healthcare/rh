use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct TypedLibrary {
    pub identifier: Option<crate::parser::ast::LibraryIdentifier>,
    pub usings: Vec<crate::parser::ast::UsingDef>,
    pub includes: Vec<crate::parser::ast::IncludeDef>,
    pub codesystems: Vec<crate::parser::ast::CodeSystemDef>,
    pub valuesets: Vec<crate::parser::ast::ValueSetDef>,
    pub codes: Vec<crate::parser::ast::CodeDef>,
    pub concepts: Vec<crate::parser::ast::ConceptDef>,
    pub parameters: Vec<crate::parser::ast::ParameterDef>,
    pub contexts: Vec<crate::parser::ast::ContextDef>,
    pub statements: Vec<TypedNode<TypedStatement>>,
}
