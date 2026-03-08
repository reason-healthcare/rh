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

    /// Set the model provider using an Arc (preferred for the new pipeline).
    pub fn set_model_provider_arc(&mut self, provider: std::sync::Arc<dyn ModelInfoProvider>) {
        self.provider = Some(provider);
    }

    /// Set the model provider from a reference.
    ///
    /// NOTE: Since `&dyn ModelInfoProvider` cannot be trivially stored as `Arc<dyn>`,
    /// this falls back to the FHIR R4 package provider. Use `set_model_provider_arc`
    /// for a custom provider in an Arc.
    pub fn set_model_provider(&mut self, _provider: &dyn ModelInfoProvider) {
        self.provider = Some(std::sync::Arc::new(
            crate::provider::fhir_r4_provider_from_package(),
        ));
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
