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
    pub compose: Option<ValueSetCompose>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetCompose {
    pub include: Option<Vec<ValueSetInclude>>,
    pub exclude: Option<Vec<ValueSetInclude>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetInclude {
    pub system: Option<String>,
    pub concept: Option<Vec<ValueSetConcept>>,
    #[serde(rename = "valueSet")]
    pub value_set: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSetConcept {
    pub code: String,
    pub display: Option<String>,
}

/// Result of checking if a code is in a ValueSet
#[derive(Debug, Clone, PartialEq)]
pub enum CodeInValueSetResult {
    /// Code is in the ValueSet
    Found,
    /// Code is not in the ValueSet
    NotFound,
    /// ValueSet could not be loaded/resolved
    ValueSetNotFound,
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

    pub fn register_valueset(&self, valueset: ValueSet) {
        let url = valueset.url.clone();
        self.cache.lock().unwrap().put(url, valueset);
    }

    pub fn load_valueset(&self, url: &str) -> Result<Option<ValueSet>> {
        let clean_url = Self::strip_version(url);

        if let Some(cached) = self.cache.lock().unwrap().get(clean_url) {
            return Ok(Some(cached.clone()));
        }

        for package_dir in &self.package_dirs {
            if let Some(valueset) = self.load_from_directory(package_dir, clean_url)? {
                // Cache ValueSets that have either expansion or compose (can be validated)
                if valueset.expansion.is_some() || valueset.compose.is_some() {
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

    pub fn contains_code(&self, url: &str, system: &str, code: &str) -> Result<CodeInValueSetResult> {
        if let Some(valueset) = self.load_valueset(url)? {
            // First check expansion (pre-expanded codes)
            if let Some(expansion) = &valueset.expansion {
                if let Some(contains) = &expansion.contains {
                    for concept in contains {
                        if concept.code == code {
                            if let Some(concept_system) = &concept.system {
                                if concept_system == system {
                                    return Ok(CodeInValueSetResult::Found);
                                }
                            } else if system.is_empty() {
                                return Ok(CodeInValueSetResult::Found);
                            }
                        }
                    }
                }
            }

            // Then check compose (intensional definition with enumerated codes)
            if let Some(compose) = &valueset.compose {
                if let Some(includes) = &compose.include {
                    for include in includes {
                        // Check if this include matches the system
                        let include_system = include.system.as_deref().unwrap_or("");
                        if !system.is_empty() && !include_system.is_empty() && include_system != system {
                            continue;
                        }

                        // Check enumerated concepts
                        if let Some(concepts) = &include.concept {
                            for concept in concepts {
                                if concept.code == code {
                                    // If system matches (or either is empty), it's a match
                                    if system.is_empty() || include_system.is_empty() || include_system == system {
                                        return Ok(CodeInValueSetResult::Found);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(CodeInValueSetResult::NotFound)
        } else {
            Ok(CodeInValueSetResult::ValueSetNotFound)
        }
    }

    pub fn contains_string_value(&self, url: &str, value: &str) -> Result<bool> {
        if let Some(valueset) = self.load_valueset(url)? {
            // Check expansion
            if let Some(expansion) = &valueset.expansion {
                if let Some(contains) = &expansion.contains {
                    for concept in contains {
                        if concept.code == value {
                            return Ok(true);
                        }
                    }
                }
            }

            // Check compose
            if let Some(compose) = &valueset.compose {
                if let Some(includes) = &compose.include {
                    for include in includes {
                        if let Some(concepts) = &include.concept {
                            for concept in concepts {
                                if concept.code == value {
                                    return Ok(true);
                                }
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

    pub fn is_valueset_url(&self, url: &str) -> bool {
        let clean_url = Self::strip_version(url);
        for package_dir in &self.package_dirs {
            if self.find_valueset_in_directory(package_dir, clean_url) {
                return true;
            }
        }
        false
    }

    fn find_valueset_in_directory(&self, dir: &Path, url: &str) -> bool {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return false,
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(valueset) = serde_json::from_str::<ValueSet>(&content) {
                        if valueset.resource_type == "ValueSet"
                            && Self::urls_match(&valueset.url, url)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn load_from_directory(&self, dir: &Path, url: &str) -> Result<Option<ValueSet>> {
        let entries = fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(valueset) = serde_json::from_str::<ValueSet>(&content) {
                        if valueset.resource_type == "ValueSet"
                            && Self::urls_match(&valueset.url, url)
                        {
                            return Ok(Some(valueset));
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
