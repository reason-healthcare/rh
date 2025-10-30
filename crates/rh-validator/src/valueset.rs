use anyhow::{Context, Result};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSet {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub url: String,
    pub status: Option<String>,
    pub expansion: Option<ValueSetExpansion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetExpansion {
    pub identifier: Option<String>,
    pub timestamp: Option<String>,
    pub total: Option<u32>,
    pub contains: Option<Vec<ValueSetContains>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetContains {
    pub system: Option<String>,
    pub code: String,
    pub display: Option<String>,
}

pub struct ValueSetLoader {
    package_dirs: Vec<PathBuf>,
    cache: Mutex<LruCache<String, ValueSet>>,
}

impl ValueSetLoader {
    pub fn new(package_dirs: Vec<PathBuf>, capacity: usize) -> Self {
        let capacity = NonZeroUsize::new(capacity).expect("Capacity must be non-zero");
        Self {
            package_dirs,
            cache: Mutex::new(LruCache::new(capacity)),
        }
    }

    pub fn load_valueset(&self, url: &str) -> Result<Option<ValueSet>> {
        let clean_url = Self::strip_version(url);

        if let Some(cached) = self.cache.lock().unwrap().get(clean_url) {
            return Ok(Some(cached.clone()));
        }

        for package_dir in &self.package_dirs {
            if let Some(valueset) = self.load_from_directory(package_dir, clean_url)? {
                if valueset.expansion.is_some() {
                    self.cache
                        .lock()
                        .unwrap()
                        .put(clean_url.to_string(), valueset.clone());
                    return Ok(Some(valueset));
                }
            }
        }

        Ok(None)
    }

    pub fn contains_code(&self, url: &str, system: &str, code: &str) -> Result<bool> {
        if let Some(valueset) = self.load_valueset(url)? {
            if let Some(expansion) = &valueset.expansion {
                if let Some(contains) = &expansion.contains {
                    for concept in contains {
                        if concept.code == code {
                            if let Some(concept_system) = &concept.system {
                                if concept_system == system {
                                    return Ok(true);
                                }
                            } else if system.is_empty() {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    pub fn is_extensional(&self, url: &str) -> Result<bool> {
        if let Some(valueset) = self.load_valueset(url)? {
            Ok(valueset.expansion.is_some())
        } else {
            Ok(false)
        }
    }

    fn load_from_directory(&self, dir: &Path, url: &str) -> Result<Option<ValueSet>> {
        let entries = fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if filename.starts_with("ValueSet-") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(valueset) = serde_json::from_str::<ValueSet>(&content) {
                                if Self::urls_match(&valueset.url, url) {
                                    return Ok(Some(valueset));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn strip_version(url: &str) -> &str {
        if let Some(pos) = url.find('|') {
            &url[..pos]
        } else {
            url
        }
    }

    fn urls_match(valueset_url: &str, requested_url: &str) -> bool {
        let vs_clean = Self::strip_version(valueset_url);
        let req_clean = Self::strip_version(requested_url);
        vs_clean == req_clean
    }

    pub fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().unwrap();
        (cache.len(), cache.cap().get())
    }
}

impl Default for ValueSetLoader {
    fn default() -> Self {
        Self::new(vec![], 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_version() {
        assert_eq!(
            ValueSetLoader::strip_version("http://example.com/ValueSet/test|1.0.0"),
            "http://example.com/ValueSet/test"
        );
        assert_eq!(
            ValueSetLoader::strip_version("http://example.com/ValueSet/test"),
            "http://example.com/ValueSet/test"
        );
    }

    #[test]
    fn test_urls_match() {
        assert!(ValueSetLoader::urls_match(
            "http://example.com/ValueSet/test",
            "http://example.com/ValueSet/test|1.0.0"
        ));
        assert!(ValueSetLoader::urls_match(
            "http://example.com/ValueSet/test|1.0.0",
            "http://example.com/ValueSet/test"
        ));
        assert!(ValueSetLoader::urls_match(
            "http://example.com/ValueSet/test|1.0.0",
            "http://example.com/ValueSet/test|2.0.0"
        ));
        assert!(!ValueSetLoader::urls_match(
            "http://example.com/ValueSet/test",
            "http://example.com/ValueSet/other"
        ));
    }
}
