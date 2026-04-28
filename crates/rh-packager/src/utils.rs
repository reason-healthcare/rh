//! Internal utility helpers shared across publisher modules.

use std::path::PathBuf;

/// Return the default FHIR packages cache directory (`$HOME/.fhir/packages`).
///
/// Falls back to `$USERPROFILE` on Windows and `/tmp` if neither is set.
pub fn default_packages_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".fhir").join("packages")
}

/// Resolve the effective packages directory from an optional processor-level override,
/// an optional global config-level override, and the built-in default.
///
/// Resolution order (first non-`None` wins):
/// 1. `processor_override` — per-processor `packages_dir` config
/// 2. `global_override` — top-level `packages_dir` in `publisher.toml`
/// 3. [`default_packages_dir()`]
pub fn resolve_packages_dir(
    processor_override: Option<&str>,
    global_override: Option<&str>,
) -> PathBuf {
    processor_override
        .or(global_override)
        .map(PathBuf::from)
        .unwrap_or_else(default_packages_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_packages_dir_prefers_processor_override() {
        let result = resolve_packages_dir(Some("/proc/pkgs"), Some("/global/pkgs"));
        assert_eq!(result, PathBuf::from("/proc/pkgs"));
    }

    #[test]
    fn resolve_packages_dir_falls_back_to_global() {
        let result = resolve_packages_dir(None, Some("/global/pkgs"));
        assert_eq!(result, PathBuf::from("/global/pkgs"));
    }

    #[test]
    fn resolve_packages_dir_falls_back_to_default() {
        let result = resolve_packages_dir(None, None);
        assert!(
            result.ends_with(".fhir/packages")
                || result == std::path::Path::new("/tmp/.fhir/packages")
        );
    }
}
