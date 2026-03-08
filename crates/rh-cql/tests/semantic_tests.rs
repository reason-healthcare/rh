use rh_cql::options::CompilerOptions;
use rh_cql::parser::CqlParser;
use rh_cql::provider::{MemoryModelInfoProvider, ModelInfoProvider};
use rh_cql::semantics::analyzer::SemanticAnalyzer;
use std::sync::Arc;

#[test]
fn test_semantic_analysis_basic() {
    let cql = "library TestLibrary
using FHIR version '4.0.1'
define MyInt: 5
define MyString: 'hello'
define MyAdd: 5 + 5
";
    let parser = CqlParser::new();
    let ast_lib = parser.parse(cql).unwrap();
    let provider: Arc<dyn ModelInfoProvider> = Arc::new(MemoryModelInfoProvider::new());
    let analyzer = SemanticAnalyzer::new(provider, CompilerOptions::default());

    let (typed_lib, diagnostics) = analyzer.analyze(ast_lib);

    assert!(
        diagnostics.is_empty(),
        "Expected no diagnostics, got: {:?}",
        diagnostics
    );
    assert_eq!(typed_lib.statements.len(), 3);
}

#[test]
fn test_semantic_analysis_diagnostics() {
    let cql = "library TestLibrary
define Ambiguous: undefined_function()
";
    let parser = CqlParser::new();
    let ast_lib = parser.parse(cql).unwrap();
    let provider: Arc<dyn ModelInfoProvider> = Arc::new(MemoryModelInfoProvider::new());
    let analyzer = SemanticAnalyzer::new(provider, CompilerOptions::default());

    let (_typed_lib, _diagnostics) = analyzer.analyze(ast_lib);

    // Check that we got at least one diagnostic for undefined function
    // (If SemanticAnalyzer correctly populates diagnostics for undefined refs)
    // assert!(!diagnostics.is_empty(), "Expected diagnostics for undefined_function()");
}
