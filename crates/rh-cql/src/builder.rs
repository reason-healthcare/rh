use crate::conversion::ConversionRegistry;
use crate::elm;
use crate::emit::ElmEmitter;
use crate::options::CompilerOptions;
use crate::parser::ast;
use crate::provider::ModelInfoProvider;
use crate::reporting::CqlCompilerException;
use crate::semantics::analyzer::SemanticAnalyzer;
use std::sync::Arc;

pub struct LibraryBuilder {
    provider: Option<Arc<dyn ModelInfoProvider>>,
    options: CompilerOptions,
    diagnostics: Vec<CqlCompilerException>,
    conversion_registry: Option<ConversionRegistry>,
}

impl Default for LibraryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LibraryBuilder {
    pub fn new() -> Self {
        Self {
            provider: None,
            options: CompilerOptions::default(),
            diagnostics: Vec::new(),
            conversion_registry: None,
        }
    }

    pub fn set_options(&mut self, options: CompilerOptions) {
        self.options = options;
    }

    pub fn set_model_provider(&mut self, _provider: &dyn ModelInfoProvider) {
        // stub mapping for integration API temporarily
    }

    pub fn set_conversion_registry(&mut self, registry: ConversionRegistry) {
        self.conversion_registry = Some(registry);
    }

    pub fn build(&mut self, library: &ast::Library) -> elm::Library {
        // Step 1: Semantic Analysis
        let real_provider = self
            .provider
            .clone()
            .unwrap_or_else(|| Arc::new(crate::provider::MemoryModelInfoProvider::default()));

        let analyzer = SemanticAnalyzer::new(real_provider, self.options.clone());
        let (typed_lib, mut analysis_diags) = analyzer.analyze(library.clone());
        self.diagnostics.append(&mut analysis_diags);

        // Step 2: ELM Emission
        let mut emitter = ElmEmitter::new(self.options.clone());

        emitter.emit(typed_lib)
    }

    pub fn errors(&self) -> &[CqlCompilerException] {
        &self.diagnostics
    }
}
