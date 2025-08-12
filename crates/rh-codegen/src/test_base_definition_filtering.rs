#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generate_organized_directories_with_traits, CodeGenerator, CodegenConfig,
        StructureDefinition,
    };
    use tempfile::TempDir;

    #[test]
    fn test_core_resource_generation() {
        let mut generator = CodeGenerator::new(CodegenConfig::default());
        let temp_dir = TempDir::new().unwrap();

        // Create a core resource (baseDefinition: DomainResource)
        let patient_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Patient".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Patient".to_string(),
            version: Some("4.0.1".to_string()),
            name: "Patient".to_string(),
            title: Some("Patient Resource".to_string()),
            status: "active".to_string(),
            description: Some("Demographics and other administrative information about an individual receiving care.".to_string()),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Patient".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/DomainResource".to_string()),
            differential: None,
            snapshot: None,
        };

        // This should succeed (core resource)
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &patient_structure,
            temp_dir.path(),
        );
        assert!(
            result.is_ok(),
            "Core resource Patient should be generated successfully"
        );
    }

    #[test]
    fn test_profile_filtering() {
        let mut generator = CodeGenerator::new(CodegenConfig::default());
        let temp_dir = TempDir::new().unwrap();

        // Create a profile resource (baseDefinition: PlanDefinition)
        let shareable_plan_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "shareableplandefinition".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/shareableplandefinition".to_string(),
            version: Some("4.0.1".to_string()),
            name: "shareableplandefinition".to_string(),
            title: Some("Shareable PlanDefinition".to_string()),
            status: "active".to_string(),
            description: Some(
                "Enforces the minimum information set for the plan definition metadata."
                    .to_string(),
            ),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "PlanDefinition".to_string(),
            base_definition: Some(
                "http://hl7.org/fhir/StructureDefinition/PlanDefinition".to_string(),
            ),
            differential: None,
            snapshot: None,
        };

        // This should be filtered out (profile of another resource)
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &shareable_plan_structure,
            temp_dir.path(),
        );
        assert!(
            result.is_err(),
            "Profile shareableplandefinition should be filtered out"
        );

        let error_message = format!("{}", result.unwrap_err());
        assert!(
            error_message.contains("only generating core resources"),
            "Error should mention filtering logic: {}",
            error_message
        );
        assert!(
            error_message.contains("PlanDefinition"),
            "Error should mention the baseDefinition: {}",
            error_message
        );
    }

    #[test]
    fn test_extension_filtering() {
        let mut generator = CodeGenerator::new(CodegenConfig::default());
        let temp_dir = TempDir::new().unwrap();

        // Create an extension (baseDefinition: Extension)
        let extension_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "patient-citizenship".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/patient-citizenship".to_string(),
            version: Some("4.0.1".to_string()),
            name: "citizenship".to_string(),
            title: Some("Citizenship".to_string()),
            status: "active".to_string(),
            description: Some("The citizen ship status of the patient.".to_string()),
            purpose: None,
            kind: "complex-type".to_string(),
            is_abstract: false,
            base_type: "Extension".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Extension".to_string()),
            differential: None,
            snapshot: None,
        };

        // This should be filtered out (extension)
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &extension_structure,
            temp_dir.path(),
        );
        assert!(result.is_err(), "Extension should be filtered out");

        let error_message = format!("{}", result.unwrap_err());
        assert!(
            error_message.contains("only generating core resources"),
            "Error should mention filtering logic: {}",
            error_message
        );
        assert!(
            error_message.contains("Extension"),
            "Error should mention the baseDefinition: {}",
            error_message
        );
    }

    #[test]
    fn test_core_base_resource_generation() {
        let mut generator = CodeGenerator::new(CodegenConfig::default());
        let temp_dir = TempDir::new().unwrap();

        // Create a base Resource (baseDefinition: Resource)
        let bundle_structure = StructureDefinition {
            resource_type: "StructureDefinition".to_string(),
            id: "Bundle".to_string(),
            url: "http://hl7.org/fhir/StructureDefinition/Bundle".to_string(),
            version: Some("4.0.1".to_string()),
            name: "Bundle".to_string(),
            title: Some("Bundle".to_string()),
            status: "active".to_string(),
            description: Some("A container for a collection of resources.".to_string()),
            purpose: None,
            kind: "resource".to_string(),
            is_abstract: false,
            base_type: "Bundle".to_string(),
            base_definition: Some("http://hl7.org/fhir/StructureDefinition/Resource".to_string()),
            differential: None,
            snapshot: None,
        };

        // This should succeed (base Resource)
        let result = generate_organized_directories_with_traits(
            &mut generator,
            &bundle_structure,
            temp_dir.path(),
        );
        assert!(
            result.is_ok(),
            "Core resource Bundle should be generated successfully"
        );
    }
}
