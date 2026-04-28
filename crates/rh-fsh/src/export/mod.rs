//! FSH export — convert tank entities to FHIR JSON resources

pub mod code_system;
pub mod instance;
pub mod mapping;
pub mod structure_def;
pub mod value_set;

use crate::error::FshError;
use crate::fhirdefs::FhirDefs;
use crate::tank::FshTank;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// A compiled FHIR package — the output of FSH compilation
pub struct FhirPackage {
    pub resources: Vec<serde_json::Value>,
    pub errors: Vec<FshError>,
}

/// FSH exporter — converts a tank into a FHIR package
pub struct FshExporter;

impl FshExporter {
    pub fn export(tank: &FshTank, defs: Arc<FhirDefs>, config: &crate::FshConfig) -> FhirPackage {
        let mut resources = Vec::new();
        let mut errors = Vec::new();

        // Pre-compute parent name → parent name chain for FHIR type resolution
        // (maps profile/extension name → its declared parent name)
        let parent_types: HashMap<String, String> = build_parent_type_map(tank);

        export_par(
            tank.profiles.values(),
            |p| structure_def::export_profile(p, defs.clone(), config, &parent_types),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.extensions.values(),
            |e| structure_def::export_extension(e, defs.clone(), config, &parent_types),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.logicals.values(),
            |l| structure_def::export_logical(l, defs.clone(), config, &parent_types),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.resources.values(),
            |r| structure_def::export_resource_def(r, defs.clone(), config, &parent_types),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.instances.values(),
            |i| instance::export_instance(i, defs.as_ref(), config, tank),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.value_sets.values(),
            |vs| value_set::export_value_set(vs, config),
            &mut resources,
            &mut errors,
        );
        export_par(
            tank.code_systems.values(),
            |cs| code_system::export_code_system(cs, config),
            &mut resources,
            &mut errors,
        );

        // Mappings do NOT produce standalone FHIR resources (H6)

        FhirPackage { resources, errors }
    }
}

/// Build a map of entity name → parent name for FHIR type resolution.
fn build_parent_type_map(tank: &FshTank) -> HashMap<String, String> {
    tank.profiles
        .iter()
        .filter_map(|(name, p)| {
            p.metadata
                .parent
                .as_ref()
                .map(|parent| (name.clone(), parent.clone()))
        })
        .chain(tank.extensions.iter().filter_map(|(name, e)| {
            e.metadata
                .parent
                .as_ref()
                .map(|parent| (name.clone(), parent.clone()))
        }))
        .collect()
}

/// Collect results, separating values from errors.
fn collect_results(
    results: Vec<Result<serde_json::Value, FshError>>,
    resources: &mut Vec<serde_json::Value>,
    errors: &mut Vec<FshError>,
) {
    for result in results {
        match result {
            Ok(v) => resources.push(v),
            Err(e) => errors.push(e),
        }
    }
}

/// Export a collection of entities in parallel, accumulating results and errors.
fn export_par<'a, T, F, I>(
    iter: I,
    f: F,
    resources: &mut Vec<serde_json::Value>,
    errors: &mut Vec<FshError>,
) where
    T: Sync + 'a,
    F: Fn(&T) -> Result<serde_json::Value, FshError> + Send + Sync,
    I: Iterator<Item = &'a T>,
{
    let items: Vec<&T> = iter.collect();
    let results: Vec<_> = items.into_par_iter().map(f).collect();
    collect_results(results, resources, errors);
}
