use crate::parser::ast;
use crate::semantics::typed_ast::TypedLibrary;
use std::fmt::Write;

pub fn explain_parse(library: &ast::Library) -> String {
    let mut out = String::new();
    out.push_str("AST Explanation:\n");
    for stmt in &library.statements {
        match stmt {
            ast::Statement::ExpressionDef(ed) => {
                writeln!(out, "ExpressionDef({})", ed.name).unwrap();
            }
            ast::Statement::FunctionDef(fd) => {
                writeln!(out, "FunctionDef({})", fd.name).unwrap();
            }
        }
    }
    out
}

pub fn explain_compile(lib: &TypedLibrary) -> String {
    let mut out = String::new();
    out.push_str("Compile Explanation:\n");
    for stmt in &lib.statements {
        writeln!(out, "TypedStatement (DataType: {:?})", stmt.data_type).unwrap();
    }
    out
}

pub fn explain_eval() -> Result<String, String> {
    Err("Evaluation engine not yet available".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_eval_stub() {
        assert_eq!(
            explain_eval(),
            Err("Evaluation engine not yet available".to_string())
        );
    }
}
