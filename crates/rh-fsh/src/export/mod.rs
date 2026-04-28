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
        let parent_types: HashMap<String, String> = tank
            .profiles
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
            .collect();

        // Profiles (parallel)
        let profile_results: Vec<_> = tank
            .profiles
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|p| structure_def::export_profile(p, defs.clone(), config, &parent_types))
            .collect();
        collect_results(profile_results, &mut resources, &mut errors);

        // Extensions (parallel)
        let ext_results: Vec<_> = tank
            .extensions
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|e| structure_def::export_extension(e, defs.clone(), config, &parent_types))
            .collect();
        collect_results(ext_results, &mut resources, &mut errors);

        // Logicals (parallel)
        let logical_results: Vec<_> = tank
            .logicals
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|l| structure_def::export_logical(l, defs.clone(), config, &parent_types))
            .collect();
        collect_results(logical_results, &mut resources, &mut errors);

        // Resources (parallel)
        let resource_results: Vec<_> = tank
            .resources
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|r| structure_def::export_resource_def(r, defs.clone(), config, &parent_types))
            .collect();
        collect_results(resource_results, &mut resources, &mut errors);

        // Instances (parallel)
        let instance_results: Vec<_> = tank
            .instances
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|i| instance::export_instance(i, defs.as_ref(), config, tank))
            .collect();
        collect_results(instance_results, &mut resources, &mut errors);

        // ValueSets (parallel)
        let vs_results: Vec<_> = tank
            .value_sets
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|vs| value_set::export_value_set(vs, config))
            .collect();
        collect_results(vs_results, &mut resources, &mut errors);

        // CodeSystems (parallel)
        let cs_results: Vec<_> = tank
            .code_systems
            .values()
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|cs| code_system::export_code_system(cs, config))
            .collect();
        collect_results(cs_results, &mut resources, &mut errors);

        // Mappings do NOT produce standalone FHIR resources (H6)

        FhirPackage { resources, errors }
    }
}

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
