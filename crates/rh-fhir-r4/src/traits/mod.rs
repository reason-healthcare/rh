//! FHIR traits for common functionality
//!
//! This module contains traits that define common interfaces for FHIR types.

// Placeholder traits - these would be generated based on FHIR structure definitions

/// Trait for types that have extensions
pub trait HasExtensions {
    /// Get the extensions for this type
    fn extensions(&self) -> &[crate::datatypes::extension::Extension];
}

/// Trait for FHIR resources
pub trait Resource {
    /// Get the resource type name
    fn resource_type(&self) -> &'static str;

    /// Get the logical id of this resource
    fn id(&self) -> Option<&str>;

    /// Get the metadata about this resource
    fn meta(&self) -> Option<&crate::datatypes::meta::Meta>;
}

/// Trait for domain resources (resources that can have narrative)
pub trait DomainResource: Resource + HasExtensions {
    /// Get the narrative text for this domain resource
    fn narrative(&self) -> Option<&crate::datatypes::narrative::Narrative>;
}
pub mod account;
pub mod activity_definition;
pub mod actualgroup;
pub mod adverse_event;
pub mod allergy_intolerance;
pub mod appointment;
pub mod appointment_response;
pub mod audit_event;
pub mod basic;
pub mod binary;
pub mod biologically_derived_product;
pub mod bmi;
pub mod body_structure;
pub mod bodyheight;
pub mod bodytemp;
pub mod bodyweight;
pub mod bp;
pub mod bundle;
pub mod capability_statement;
pub mod care_plan;
pub mod care_team;
pub mod catalog;
pub mod catalog_entry;
pub mod cdshooksguidanceresponse;
pub mod cdshooksrequestgroup;
pub mod cdshooksserviceplandefinition;
pub mod charge_item;
pub mod charge_item_definition;
pub mod cholesterol;
pub mod claim;
pub mod claim_response;
pub mod clinical_impression;
pub mod clinicaldocument;
pub mod code_system;
pub mod communication;
pub mod communication_request;
pub mod compartment_definition;
pub mod composition;
pub mod computableplandefinition;
pub mod concept_map;
pub mod condition;
pub mod consent;
pub mod contract;
pub mod coverage;
pub mod coverage_eligibility_request;
pub mod coverage_eligibility_response;
pub mod cqf_questionnaire;
pub mod cqllibrary;
pub mod detected_issue;
pub mod device;
pub mod device_definition;
pub mod device_metric;
pub mod device_request;
pub mod device_use_statement;
pub mod devicemetricobservation;
pub mod diagnostic_report;
pub mod diagnosticreport_genetics;
pub mod document_manifest;
pub mod document_reference;
pub mod domain_resource;
pub mod effect_evidence_synthesis;
pub mod ehrsrle_auditevent;
pub mod ehrsrle_provenance;
pub mod encounter;
pub mod endpoint;
pub mod enrollment_request;
pub mod enrollment_response;
pub mod episode_of_care;
pub mod event_definition;
pub mod evidence;
pub mod evidence_variable;
pub mod explanation_of_benefit;
pub mod family_member_history;
pub mod familymemberhistory_genetic;
pub mod flag;
pub mod goal;
pub mod graph_definition;
pub mod group;
pub mod groupdefinition;
pub mod guidance_response;
pub mod hdlcholesterol;
pub mod headcircum;
pub mod healthcare_service;
pub mod heartrate;
pub mod hlaresult;
pub mod imaging_study;
pub mod immunization;
pub mod immunization_evaluation;
pub mod immunization_recommendation;
pub mod implementation_guide;
pub mod insurance_plan;
pub mod invoice;
pub mod ldlcholesterol;
pub mod library;
pub mod linkage;
pub mod lipidprofile;
pub mod list;
pub mod location;
pub mod measure;
pub mod measure_report;
pub mod media;
pub mod medication;
pub mod medication_administration;
pub mod medication_dispense;
pub mod medication_knowledge;
pub mod medication_request;
pub mod medication_statement;
pub mod medicinal_product;
pub mod medicinal_product_authorization;
pub mod medicinal_product_contraindication;
pub mod medicinal_product_indication;
pub mod medicinal_product_ingredient;
pub mod medicinal_product_interaction;
pub mod medicinal_product_manufactured;
pub mod medicinal_product_packaged;
pub mod medicinal_product_pharmaceutical;
pub mod medicinal_product_undesirable_effect;
pub mod message_definition;
pub mod message_header;
pub mod molecular_sequence;
pub mod naming_system;
pub mod nutrition_order;
pub mod observation;
pub mod observation_definition;
pub mod observation_genetics;
pub mod operation_definition;
pub mod operation_outcome;
pub mod organization;
pub mod organization_affiliation;
pub mod oxygensat;
pub mod parameters;
pub mod patient;
pub mod payment_notice;
pub mod payment_reconciliation;
pub mod person;
pub mod picoelement;
pub mod plan_definition;
pub mod practitioner;
pub mod practitioner_role;
pub mod procedure;
pub mod provenance;
pub mod provenance_relevant_history;
pub mod questionnaire;
pub mod questionnaire_response;
pub mod related_person;
pub mod request_group;
pub mod research_definition;
pub mod research_element_definition;
pub mod research_study;
pub mod research_subject;
pub mod resource;
pub mod resprate;
pub mod risk_assessment;
pub mod risk_evidence_synthesis;
pub mod schedule;
pub mod search_parameter;
pub mod service_request;
pub mod servicerequest_genetics;
pub mod shareableactivitydefinition;
pub mod shareablecodesystem;
pub mod shareablelibrary;
pub mod shareablemeasure;
pub mod shareableplandefinition;
pub mod shareablevalueset;
pub mod slot;
pub mod specimen;
pub mod specimen_definition;
pub mod structure_definition;
pub mod structure_map;
pub mod subscription;
pub mod substance;
pub mod substance_nucleic_acid;
pub mod substance_polymer;
pub mod substance_protein;
pub mod substance_reference_information;
pub mod substance_source_material;
pub mod substance_specification;
pub mod supply_delivery;
pub mod supply_request;
pub mod synthesis;
pub mod task;
pub mod terminology_capabilities;
pub mod test_report;
pub mod test_script;
pub mod triglyceride;
pub mod value_set;
pub mod verification_result;
pub mod vision_prescription;
pub mod vitalsigns;
pub mod vitalspanel;
