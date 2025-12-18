use anyhow::{Context, Result};
use lru::LruCache;
use rh_foundation::loader::PackageLoader;
use rh_foundation::snapshot::{SnapshotGenerator, StructureDefinition, StructureDefinitionLoader};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::fhir_version::FhirVersion;

pub struct ProfileRegistry {
    generator: SnapshotGenerator,
    profiles: HashMap<String, StructureDefinition>,
    /// Dynamically registered profiles (can be added at runtime with &self)
    dynamic_profiles: RwLock<HashMap<String, StructureDefinition>>,
    snapshot_cache: RwLock<LruCache<String, StructureDefinition>>,
    cache_hits: RwLock<usize>,
    cache_misses: RwLock<usize>,
    #[allow(dead_code)]
    fhir_version: FhirVersion,
}

impl ProfileRegistry {
    pub fn new(fhir_version: FhirVersion, packages_dir: Option<&str>) -> Result<Self> {
        let mut generator = SnapshotGenerator::new();
        let mut profiles = HashMap::new();

        // Try to load core FHIR package from default packages directory
        // Package directory format: ~/.fhir/packages/hl7.fhir.r4.core#4.0.1/package
        if let Ok(default_dir) = PackageLoader::get_default_packages_dir() {
            let package_name_with_version = format!(
                "{}#{}",
                fhir_version.package_id(),
                fhir_version.version_string()
            );
            let core_package_dir = default_dir.join(&package_name_with_version).join("package");

            if core_package_dir.exists() {
                match StructureDefinitionLoader::load_from_directory(&core_package_dir) {
                    Ok(loaded_profiles) => {
                        for profile in loaded_profiles {
                            profiles.insert(profile.url.clone(), profile.clone());
                            generator.load_structure_definition(profile);
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to load core FHIR package from {}: {}",
                            core_package_dir.display(),
                            e
                        );
                    }
                }
            } else {
                eprintln!(
                    "Warning: Core FHIR package not found at {}. Some validations may fail.",
                    core_package_dir.display()
                );
            }
        }

        // Load additional profiles from packages directory if provided
        let packages_path = packages_dir.map(PathBuf::from);
        if let Some(ref dir) = packages_path {
            if dir.exists() {
                let loaded_profiles = StructureDefinitionLoader::load_from_directory(dir)
                    .context("Failed to load profiles from packages directory")?;

                for profile in loaded_profiles {
                    profiles.insert(profile.url.clone(), profile.clone());
                    generator.load_structure_definition(profile);
                }
            }
        }

        let capacity = NonZeroUsize::new(100).unwrap();
        Ok(Self {
            generator,
            profiles,
            dynamic_profiles: RwLock::new(HashMap::new()),
            snapshot_cache: RwLock::new(LruCache::new(capacity)),
            cache_hits: RwLock::new(0),
            cache_misses: RwLock::new(0),
            fhir_version,
        })
    }

    pub fn get_snapshot(&self, profile_url: &str) -> Result<Option<StructureDefinition>> {
        if let Some(cached) = self.snapshot_cache.write().unwrap().get(profile_url) {
            *self.cache_hits.write().unwrap() += 1;
            return Ok(Some(cached.clone()));
        }

        *self.cache_misses.write().unwrap() += 1;

        // First check dynamically registered profiles
        if let Some(profile) = self
            .dynamic_profiles
            .read()
            .unwrap()
            .get(profile_url)
            .cloned()
        {
            self.snapshot_cache
                .write()
                .unwrap()
                .put(profile_url.to_string(), profile.clone());
            return Ok(Some(profile));
        }

        // Then check statically loaded profiles
        if !self.profiles.contains_key(profile_url) {
            return Ok(None);
        }

        let snapshot = self
            .generator
            .generate_snapshot(profile_url)
            .context("Failed to generate snapshot")?;

        let profile = self.profiles.get(profile_url).unwrap().clone();
        let mut profile_with_snapshot = profile;
        profile_with_snapshot.snapshot = Some((*snapshot).clone());

        self.snapshot_cache
            .write()
            .unwrap()
            .put(profile_url.to_string(), profile_with_snapshot.clone());

        Ok(Some(profile_with_snapshot))
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn search_profiles(&self, query: &str) -> Vec<String> {
        let query_lower = query.to_lowercase();
        self.profiles
            .iter()
            .filter(|(url, profile)| {
                url.to_lowercase().contains(&query_lower)
                    || profile.name.to_lowercase().contains(&query_lower)
            })
            .map(|(url, _)| url.clone())
            .collect()
    }

    /// Register a profile dynamically (can be called with &self)
    /// Note: Dynamically registered profiles don't support snapshot generation
    /// since the SnapshotGenerator isn't thread-safe. The profile is stored
    /// as-is, which is sufficient for checking extension isModifier flags.
    pub fn register_profile(&self, profile: StructureDefinition) {
        self.dynamic_profiles
            .write()
            .unwrap()
            .insert(profile.url.clone(), profile);
    }

    pub fn load_profile(&mut self, profile: StructureDefinition) -> Result<()> {
        self.profiles.insert(profile.url.clone(), profile.clone());
        self.generator.load_structure_definition(profile);
        Ok(())
    }

    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        let profile = StructureDefinitionLoader::load_from_file(std::path::Path::new(path))
            .context("Failed to load profile from file")?;
        self.load_profile(profile)
    }

    pub fn load_from_directory(&mut self, path: &str) -> Result<()> {
        let profiles = StructureDefinitionLoader::load_from_directory(std::path::Path::new(path))
            .context("Failed to load profiles from directory")?;
        for profile in profiles {
            self.load_profile(profile)?;
        }
        Ok(())
    }

    pub fn cache_stats(&self) -> (usize, usize) {
        let cache = self.snapshot_cache.read().unwrap();
        (cache.len(), cache.cap().get())
    }

    pub fn cache_metrics(&self) -> (usize, usize, f64) {
        let hits = *self.cache_hits.read().unwrap();
        let misses = *self.cache_misses.read().unwrap();
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }

    pub fn reset_cache_metrics(&self) {
        *self.cache_hits.write().unwrap() = 0;
        *self.cache_misses.write().unwrap() = 0;
    }

    pub fn extract_profile_urls(resource: &serde_json::Value) -> Vec<String> {
        let mut urls = Vec::new();

        if let Some(meta) = resource.get("meta") {
            if let Some(profile) = meta.get("profile") {
                if let Some(arr) = profile.as_array() {
                    for url in arr {
                        if let Some(url_str) = url.as_str() {
                            urls.push(url_str.to_string());
                        }
                    }
                }
            }
        }

        urls
    }
}

impl Default for ProfileRegistry {
    fn default() -> Self {
        Self::new(FhirVersion::default(), None).expect("Failed to initialize ProfileRegistry")
    }
}
