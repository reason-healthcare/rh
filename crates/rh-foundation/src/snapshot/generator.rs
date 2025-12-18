use crate::snapshot::error::{SnapshotError, SnapshotResult};
use crate::snapshot::merger::ElementMerger;
use crate::snapshot::types::{Snapshot, StructureDefinition};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::{debug, info};

pub struct SnapshotGenerator {
    structure_definitions: HashMap<String, StructureDefinition>,
    snapshot_cache: RefCell<HashMap<String, Arc<Snapshot>>>,
}

impl SnapshotGenerator {
    pub fn new() -> Self {
        Self {
            structure_definitions: HashMap::new(),
            snapshot_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn clear_cache(&self) {
        self.snapshot_cache.borrow_mut().clear();
    }

    pub fn cache_size(&self) -> usize {
        self.snapshot_cache.borrow().len()
    }

    pub fn load_structure_definition(&mut self, sd: StructureDefinition) {
        debug!("Loading StructureDefinition: {} ({})", sd.name, sd.url);
        self.structure_definitions.insert(sd.url.clone(), sd);
    }

    pub fn load_structure_definitions(&mut self, sds: Vec<StructureDefinition>) {
        for sd in sds {
            self.load_structure_definition(sd);
        }
    }

    pub fn generate_snapshot(&self, url: &str) -> SnapshotResult<Arc<Snapshot>> {
        info!("Generating snapshot for: {}", url);
        let mut visited = HashSet::new();
        self.generate_snapshot_internal(url, &mut visited)
    }

    fn generate_snapshot_internal(
        &self,
        url: &str,
        visited: &mut HashSet<String>,
    ) -> SnapshotResult<Arc<Snapshot>> {
        if visited.contains(url) {
            return Err(SnapshotError::CircularDependency(format!(
                "Circular dependency detected: {url}"
            )));
        }

        if let Some(cached) = self.snapshot_cache.borrow().get(url) {
            debug!("Using cached snapshot for {}", url);
            return Ok(cached.clone());
        }

        visited.insert(url.to_string());

        let sd = self
            .structure_definitions
            .get(url)
            .ok_or_else(|| SnapshotError::BaseNotFound(url.to_string()))?;

        debug!(
            "Processing StructureDefinition: {} (type: {})",
            sd.name, sd.type_
        );

        if let Some(ref snapshot) = sd.snapshot {
            debug!(
                "Using existing snapshot with {} elements",
                snapshot.element.len()
            );
            let snapshot = Arc::new(snapshot.clone());
            self.snapshot_cache
                .borrow_mut()
                .insert(url.to_string(), snapshot.clone());
            return Ok(snapshot);
        }

        let snapshot = if let Some(base_url) = &sd.base_definition {
            debug!("Resolving base definition: {}", base_url);
            let base_snapshot = self.generate_snapshot_internal(base_url, visited)?;

            if let Some(differential) = &sd.differential {
                debug!(
                    "Merging {} base elements with {} differential elements",
                    base_snapshot.element.len(),
                    differential.element.len()
                );
                let merged =
                    ElementMerger::merge_elements(&base_snapshot.element, &differential.element)?;
                info!("Generated snapshot with {} elements", merged.len());
                Arc::new(Snapshot { element: merged })
            } else {
                debug!("No differential, returning base snapshot");
                base_snapshot
            }
        } else if let Some(differential) = &sd.differential {
            debug!(
                "No base definition, using differential as snapshot ({} elements)",
                differential.element.len()
            );
            Arc::new(Snapshot {
                element: differential.element.clone(),
            })
        } else {
            return Err(SnapshotError::InvalidStructureDefinition(format!(
                "StructureDefinition {url} has no snapshot, differential, or base definition"
            )));
        };

        self.snapshot_cache
            .borrow_mut()
            .insert(url.to_string(), snapshot.clone());
        Ok(snapshot)
    }
}

impl Default for SnapshotGenerator {
    fn default() -> Self {
        Self::new()
    }
}
