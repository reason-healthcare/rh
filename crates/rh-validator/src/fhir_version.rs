use std::fmt;

/// FHIR version specification
///
/// This enum represents the different versions of FHIR that can be validated.
/// Each variant provides version-specific configuration like package IDs and URLs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FhirVersion {
    /// FHIR R4 (v4.0.1)
    #[default]
    R4,
}

impl FhirVersion {
    /// Returns the FHIR package ID for this version.
    pub fn package_id(&self) -> &'static str {
        match self {
            FhirVersion::R4 => "hl7.fhir.r4.core",
        }
    }

    /// Returns the version string for this FHIR version.
    pub fn version_string(&self) -> &'static str {
        match self {
            FhirVersion::R4 => "4.0.1",
        }
    }

    /// Returns the base URL for resource profiles in this version.
    pub fn base_url(&self) -> &'static str {
        match self {
            FhirVersion::R4 => "http://hl7.org/fhir/StructureDefinition",
        }
    }
}

impl fmt::Display for FhirVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FhirVersion::R4 => write!(f, "R4 (4.0.1)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_id() {
        assert_eq!(FhirVersion::R4.package_id(), "hl7.fhir.r4.core");
    }

    #[test]
    fn test_version_string() {
        assert_eq!(FhirVersion::R4.version_string(), "4.0.1");
    }

    #[test]
    fn test_base_url() {
        assert_eq!(
            FhirVersion::R4.base_url(),
            "http://hl7.org/fhir/StructureDefinition"
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(FhirVersion::default(), FhirVersion::R4);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FhirVersion::R4), "R4 (4.0.1)");
    }
}
