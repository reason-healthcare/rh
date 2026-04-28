//! Lifecycle hook runner and `HookProcessor` trait.
//!
//! Processors are registered by name in a `ProcessorRegistry`. The runner
//! executes the processors declared for a given stage in order, aborting on
//! the first failure.

use crate::{context::PublishContext, PublisherError, Result};
use std::collections::HashMap;

/// A lifecycle hook processor that can be registered and run at a pipeline stage.
pub trait HookProcessor: Send + Sync {
    /// Unique name used to reference this processor in `packager.toml`.
    fn name(&self) -> &'static str;

    /// Execute the processor against the current publish context.
    ///
    /// Processors may mutate `ctx.resources` (adding or modifying entries) but
    /// must not write to disk directly — output is written by the pack stage.
    fn run(&self, ctx: &mut PublishContext) -> Result<()>;
}

/// Build the default registry with all built-in processors registered.
pub fn build_registry() -> ProcessorRegistry {
    use crate::processors::{
        cql::CqlProcessor, snapshot::SnapshotProcessor, validate::ValidateProcessor,
    };
    let mut registry = ProcessorRegistry::new();
    registry.register(SnapshotProcessor);
    registry.register(ValidateProcessor);
    registry.register(CqlProcessor);
    registry
}

/// Registry mapping processor names to their implementations.
pub struct ProcessorRegistry {
    processors: HashMap<&'static str, Box<dyn HookProcessor>>,
}

impl ProcessorRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            processors: HashMap::new(),
        }
    }

    /// Register a processor. Overwrites any existing registration with the same name.
    pub fn register(&mut self, processor: impl HookProcessor + 'static) {
        self.processors
            .insert(processor.name(), Box::new(processor));
    }

    /// Resolve processor names to their implementations, returning an error if any
    /// name is not registered.
    fn resolve<'a>(&'a self, names: &[String]) -> Result<Vec<&'a dyn HookProcessor>> {
        names
            .iter()
            .map(|name| {
                self.processors
                    .get(name.as_str())
                    .map(|p| p.as_ref())
                    .ok_or_else(|| PublisherError::UnknownProcessor(name.clone()))
            })
            .collect()
    }
}

impl Default for ProcessorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Run the processors declared for `stage_processors` in order, aborting on the
/// first error. All processor names must be registered in `registry`.
///
/// # Errors
/// - `PublisherError::UnknownProcessor` if any name in `stage_processors` is not registered.
/// - The first `PublisherError` returned by any processor.
pub fn run_stage(
    registry: &ProcessorRegistry,
    stage_processors: &[String],
    ctx: &mut PublishContext,
) -> Result<()> {
    // Resolve all names up-front so we fail fast before touching any data.
    let resolved = registry.resolve(stage_processors)?;

    for processor in resolved {
        processor
            .run(ctx)
            .map_err(|e| PublisherError::HookProcessor {
                processor: processor.name().to_string(),
                message: e.to_string(),
            })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::PublishContext;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    fn empty_ctx() -> PublishContext {
        PublishContext::for_testing("/tmp/src")
    }

    struct CountingProcessor {
        counter: Arc<AtomicUsize>,
    }

    impl HookProcessor for CountingProcessor {
        fn name(&self) -> &'static str {
            "counter"
        }
        fn run(&self, _ctx: &mut PublishContext) -> Result<()> {
            self.counter.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct FailingProcessor;

    impl HookProcessor for FailingProcessor {
        fn name(&self) -> &'static str {
            "fail"
        }
        fn run(&self, _ctx: &mut PublishContext) -> Result<()> {
            Err(crate::PublisherError::ValidationFailed(
                "intentional".to_string(),
            ))
        }
    }

    #[test]
    fn unknown_processor_name_returns_error_before_execution() {
        let registry = ProcessorRegistry::new();
        let mut ctx = empty_ctx();
        let stages = vec!["nonexistent".to_string()];
        let err = run_stage(&registry, &stages, &mut ctx).unwrap_err();
        assert!(matches!(err, PublisherError::UnknownProcessor(n) if n == "nonexistent"));
    }

    #[test]
    fn processors_run_in_order() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut registry = ProcessorRegistry::new();
        registry.register(CountingProcessor {
            counter: counter.clone(),
        });

        let mut ctx = empty_ctx();
        let stages = vec![
            "counter".to_string(),
            "counter".to_string(),
            "counter".to_string(),
        ];
        run_stage(&registry, &stages, &mut ctx).unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn aborts_on_first_failure() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut registry = ProcessorRegistry::new();
        registry.register(CountingProcessor {
            counter: counter.clone(),
        });
        registry.register(FailingProcessor);

        let mut ctx = empty_ctx();
        let stages = vec![
            "counter".to_string(),
            "fail".to_string(),
            "counter".to_string(), // should NOT be reached
        ];
        let err = run_stage(&registry, &stages, &mut ctx).unwrap_err();
        // The counter should only have been incremented once (before the failure).
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert!(matches!(err, PublisherError::HookProcessor { .. }));
    }

    #[test]
    fn empty_stage_list_succeeds() {
        let registry = ProcessorRegistry::new();
        let mut ctx = empty_ctx();
        run_stage(&registry, &[], &mut ctx).unwrap();
    }

    #[test]
    fn hook_processor_error_wraps_processor_name() {
        let mut registry = ProcessorRegistry::new();
        registry.register(FailingProcessor);

        let mut ctx = empty_ctx();
        let err = run_stage(&registry, &["fail".to_string()], &mut ctx).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("fail"),
            "Expected processor name 'fail' in error: {msg}"
        );
    }
}
