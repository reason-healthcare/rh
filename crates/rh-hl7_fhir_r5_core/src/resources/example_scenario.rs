use crate::bindings::examplescenario_actor_type::ExamplescenarioActorType;
use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::coding::Coding;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::reference::Reference;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ExampleScenario
///
/// A walkthrough of a workflow showing the interaction between systems and the instances shared, possibly including the evolution of instances over time.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ExampleScenario
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ExampleScenario
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenario {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Canonical identifier for this example scenario, represented as a URI (globally unique)
    pub url: Option<StringType>,
    /// Extension element for the 'url' primitive field. Contains metadata and extensions.
    pub _url: Option<Element>,
    /// Additional identifier for the example scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// Business version of the example scenario
    pub version: Option<StringType>,
    /// Extension element for the 'version' primitive field. Contains metadata and extensions.
    pub _version: Option<Element>,
    /// How to compare versions (string)
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<StringType>,
    /// How to compare versions (Coding)
    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,
    /// To be removed?
    pub name: Option<StringType>,
    /// Extension element for the 'name' primitive field. Contains metadata and extensions.
    pub _name: Option<Element>,
    /// Name for this example scenario (human friendly)
    pub title: Option<StringType>,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// draft | active | retired | unknown
    pub status: PublicationStatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// For testing purposes, not real usage
    pub experimental: Option<BooleanType>,
    /// Extension element for the 'experimental' primitive field. Contains metadata and extensions.
    pub _experimental: Option<Element>,
    /// Date last changed
    pub date: Option<DateTimeType>,
    /// Extension element for the 'date' primitive field. Contains metadata and extensions.
    pub _date: Option<Element>,
    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<StringType>,
    /// Extension element for the 'publisher' primitive field. Contains metadata and extensions.
    pub _publisher: Option<Element>,
    /// Contact details for the publisher
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactDetail>,
    /// Natural language description of the ExampleScenario
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub use_context: Vec<UsageContext>,
    /// Intended jurisdiction for example scenario (if applicable)
    ///
    /// Binding: extensible (Countries and regions within which this artifact is targeted for use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/jurisdiction
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jurisdiction: Vec<CodeableConcept>,
    /// The purpose of the example, e.g. to illustrate a scenario
    pub purpose: Option<StringType>,
    /// Extension element for the 'purpose' primitive field. Contains metadata and extensions.
    pub _purpose: Option<Element>,
    /// Use and/or publishing restrictions
    pub copyright: Option<StringType>,
    /// Extension element for the 'copyright' primitive field. Contains metadata and extensions.
    pub _copyright: Option<Element>,
    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<StringType>,
    /// Extension element for the 'copyrightLabel' primitive field. Contains metadata and extensions.
    #[serde(rename = "_copyrightLabel")]
    pub _copyright_label: Option<Element>,
    /// Individual involved in exchange
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor: Vec<ExampleScenarioActor>,
    /// Data used in the scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instance: Vec<ExampleScenarioInstance>,
    /// Major process within scenario
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub process: Vec<ExampleScenarioProcess>,
}
/// ExampleScenario nested structure for the 'actor' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioActor {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// ID or acronym of the actor
    pub key: StringType,
    /// Extension element for the 'key' primitive field. Contains metadata and extensions.
    pub _key: Option<Element>,
    /// person | system
    #[serde(rename = "type")]
    pub type_: ExamplescenarioActorType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// Label for actor when rendering
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Details about actor
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
}
/// ExampleScenario nested structure for the 'instance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioInstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Resources contained in the instance
    #[serde(rename = "containedInstance")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contained_instance: Vec<ExampleScenarioInstanceContainedinstance>,
    /// Snapshot of instance that changes
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub version: Vec<ExampleScenarioInstanceVersion>,
    /// ID or acronym of the instance
    pub key: StringType,
    /// Extension element for the 'key' primitive field. Contains metadata and extensions.
    pub _key: Option<Element>,
    /// Data structure for example
    ///
    /// Binding: extensible (The structure that defines the instance)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/examplescenario-instance-type
    #[serde(rename = "structureType")]
    pub structure_type: Coding,
    /// E.g. 4.0.1
    #[serde(rename = "structureVersion")]
    pub structure_version: Option<StringType>,
    /// Extension element for the 'structureVersion' primitive field. Contains metadata and extensions.
    #[serde(rename = "_structureVersion")]
    pub _structure_version: Option<Element>,
    /// Rules instance adheres to (canonical)
    #[serde(rename = "structureProfileCanonical")]
    pub structure_profile_canonical: Option<StringType>,
    /// Rules instance adheres to (uri)
    #[serde(rename = "structureProfileUri")]
    pub structure_profile_uri: Option<StringType>,
    /// Label for instance
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Human-friendly description of the instance
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Example instance data
    pub content: Option<Reference>,
}
/// ExampleScenarioInstance nested structure for the 'containedInstance' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioInstanceContainedinstance {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Key of contained instance
    #[serde(rename = "instanceReference")]
    pub instance_reference: StringType,
    /// Extension element for the 'instanceReference' primitive field. Contains metadata and extensions.
    #[serde(rename = "_instanceReference")]
    pub _instance_reference: Option<Element>,
    /// Key of contained instance version
    #[serde(rename = "versionReference")]
    pub version_reference: Option<StringType>,
    /// Extension element for the 'versionReference' primitive field. Contains metadata and extensions.
    #[serde(rename = "_versionReference")]
    pub _version_reference: Option<Element>,
}
/// ExampleScenarioInstance nested structure for the 'version' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioInstanceVersion {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// ID or acronym of the version
    pub key: StringType,
    /// Extension element for the 'key' primitive field. Contains metadata and extensions.
    pub _key: Option<Element>,
    /// Label for instance version
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Details about version
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Example instance version data
    pub content: Option<Reference>,
}
/// ExampleScenario nested structure for the 'process' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioProcess {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Event within of the process
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<ExampleScenarioProcessStep>,
    /// Label for procss
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Human-friendly description of the process
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Status before process starts
    #[serde(rename = "preConditions")]
    pub pre_conditions: Option<StringType>,
    /// Extension element for the 'preConditions' primitive field. Contains metadata and extensions.
    #[serde(rename = "_preConditions")]
    pub _pre_conditions: Option<Element>,
    /// Status after successful completion
    #[serde(rename = "postConditions")]
    pub post_conditions: Option<StringType>,
    /// Extension element for the 'postConditions' primitive field. Contains metadata and extensions.
    #[serde(rename = "_postConditions")]
    pub _post_conditions: Option<Element>,
}
/// ExampleScenarioProcess nested structure for the 'step' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioProcessStep {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Sequential number of the step
    pub number: Option<StringType>,
    /// Extension element for the 'number' primitive field. Contains metadata and extensions.
    pub _number: Option<Element>,
    /// Step is nested process
    pub process: Option<StringType>,
    /// Step is nested workflow
    pub workflow: Option<StringType>,
    /// Extension element for the 'workflow' primitive field. Contains metadata and extensions.
    pub _workflow: Option<Element>,
    /// Pause in the flow?
    pub pause: Option<BooleanType>,
    /// Extension element for the 'pause' primitive field. Contains metadata and extensions.
    pub _pause: Option<Element>,
}
/// ExampleScenarioProcessStep nested structure for the 'alternative' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioProcessStepAlternative {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Label for alternative
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Human-readable description of option
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Alternative action(s)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step: Vec<StringType>,
}
/// ExampleScenarioProcessStep nested structure for the 'operation' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleScenarioProcessStepOperation {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Kind of action
    ///
    /// Binding: extensible (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/testscript-operation-codes
    #[serde(rename = "type")]
    pub type_: Option<Coding>,
    /// Label for step
    pub title: StringType,
    /// Extension element for the 'title' primitive field. Contains metadata and extensions.
    pub _title: Option<Element>,
    /// Who starts the operation
    pub initiator: Option<StringType>,
    /// Extension element for the 'initiator' primitive field. Contains metadata and extensions.
    pub _initiator: Option<Element>,
    /// Who receives the operation
    pub receiver: Option<StringType>,
    /// Extension element for the 'receiver' primitive field. Contains metadata and extensions.
    pub _receiver: Option<Element>,
    /// Human-friendly description of the operation
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Initiator stays active?
    #[serde(rename = "initiatorActive")]
    pub initiator_active: Option<BooleanType>,
    /// Extension element for the 'initiatorActive' primitive field. Contains metadata and extensions.
    #[serde(rename = "_initiatorActive")]
    pub _initiator_active: Option<Element>,
    /// Receiver stays active?
    #[serde(rename = "receiverActive")]
    pub receiver_active: Option<BooleanType>,
    /// Extension element for the 'receiverActive' primitive field. Contains metadata and extensions.
    #[serde(rename = "_receiverActive")]
    pub _receiver_active: Option<Element>,
    /// Instance transmitted on invocation
    pub request: Option<StringType>,
    /// Instance transmitted on invocation response
    pub response: Option<StringType>,
}

impl Default for ExampleScenario {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            url: Default::default(),
            _url: Default::default(),
            identifier: Default::default(),
            version: Default::default(),
            _version: Default::default(),
            version_algorithm_string: Default::default(),
            version_algorithm_coding: Default::default(),
            name: Default::default(),
            _name: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            status: PublicationStatus::default(),
            _status: Default::default(),
            experimental: Default::default(),
            _experimental: Default::default(),
            date: Default::default(),
            _date: Default::default(),
            publisher: Default::default(),
            _publisher: Default::default(),
            contact: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            use_context: Default::default(),
            jurisdiction: Default::default(),
            purpose: Default::default(),
            _purpose: Default::default(),
            copyright: Default::default(),
            _copyright: Default::default(),
            copyright_label: Default::default(),
            _copyright_label: Default::default(),
            actor: Default::default(),
            instance: Default::default(),
            process: Default::default(),
        }
    }
}

impl Default for ExampleScenarioActor {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            key: StringType::default(),
            _key: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            title: StringType::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
        }
    }
}

impl Default for ExampleScenarioInstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            contained_instance: Default::default(),
            version: Default::default(),
            key: StringType::default(),
            _key: Default::default(),
            structure_type: Coding::default(),
            structure_version: Default::default(),
            _structure_version: Default::default(),
            structure_profile_canonical: Default::default(),
            structure_profile_uri: Default::default(),
            title: StringType::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            content: Default::default(),
        }
    }
}

impl Default for ExampleScenarioInstanceContainedinstance {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            instance_reference: Default::default(),
            _instance_reference: Default::default(),
            version_reference: Default::default(),
            _version_reference: Default::default(),
        }
    }
}

impl Default for ExampleScenarioInstanceVersion {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            key: Default::default(),
            _key: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            content: Default::default(),
        }
    }
}

impl Default for ExampleScenarioProcess {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            step: Default::default(),
            title: StringType::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            pre_conditions: Default::default(),
            _pre_conditions: Default::default(),
            post_conditions: Default::default(),
            _post_conditions: Default::default(),
        }
    }
}

impl Default for ExampleScenarioProcessStep {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            number: Default::default(),
            _number: Default::default(),
            process: Default::default(),
            workflow: Default::default(),
            _workflow: Default::default(),
            pause: Default::default(),
            _pause: Default::default(),
        }
    }
}

impl Default for ExampleScenarioProcessStepAlternative {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            title: Default::default(),
            _title: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            step: Default::default(),
        }
    }
}

impl Default for ExampleScenarioProcessStepOperation {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            title: Default::default(),
            _title: Default::default(),
            initiator: Default::default(),
            _initiator: Default::default(),
            receiver: Default::default(),
            _receiver: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            initiator_active: Default::default(),
            _initiator_active: Default::default(),
            receiver_active: Default::default(),
            _receiver_active: Default::default(),
            request: Default::default(),
            response: Default::default(),
        }
    }
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("cnl-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.exists() implies name.matches('^[A-Z]([A-Za-z0-9_]){1,254}$')"),
    rh_foundation::Invariant::new("cnl-1", rh_foundation::Severity::Warning, "URL should not contain | or # - these characters make processing canonical references problematic", "exists() implies matches('^[^|# ]+$')"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
    rh_foundation::Invariant::new("exs-1", rh_foundation::Severity::Error, "StructureVersion is required if structureType is not FHIR (but may still be present even if FHIR)", "structureType.exists() and structureType.memberOf('http://hl7.org/fhir/ValueSet/resource-types').not() implies structureVersion.exists()"),
    rh_foundation::Invariant::new("exs-10", rh_foundation::Severity::Error, "Version keys must be unique within an instance", "version.key.count() = version.key.distinct().count()"),
    rh_foundation::Invariant::new("exs-11", rh_foundation::Severity::Error, "Version titles must be unique within an instance", "version.title.count() = version.title.distinct().count()"),
    rh_foundation::Invariant::new("exs-12", rh_foundation::Severity::Error, "Process titles must be unique", "process.title.count() = process.title.distinct().count()"),
    rh_foundation::Invariant::new("exs-13", rh_foundation::Severity::Error, "Alternative titles must be unique within a step", "alternative.title.count() = alternative.title.distinct().count()"),
    rh_foundation::Invariant::new("exs-14", rh_foundation::Severity::Error, "InstanceReference must be a key of an instance defined in the ExampleScenario", "%resource.instance.where(key=%context.instanceReference).exists()"),
    rh_foundation::Invariant::new("exs-15", rh_foundation::Severity::Error, "versionReference must be specified if the referenced instance defines versions", "versionReference.empty() implies %resource.instance.where(key=%context.instanceReference).version.empty()"),
    rh_foundation::Invariant::new("exs-16", rh_foundation::Severity::Error, "versionReference must be a key of a version within the instance pointed to by instanceReference", "versionReference.exists() implies %resource.instance.where(key=%context.instanceReference).version.where(key=%context.versionReference).exists()"),
    rh_foundation::Invariant::new("exs-17", rh_foundation::Severity::Error, "If specified, initiator must be a key of an actor within the ExampleScenario", "initiator.exists() implies initiator = 'OTHER' or %resource.actor.where(key=%context.initiator).exists()"),
    rh_foundation::Invariant::new("exs-18", rh_foundation::Severity::Error, "If specified, receiver must be a key of an actor within the ExampleScenario", "receiver.exists() implies receiver = 'OTHER' or %resource.actor.where(key=%context.receiver).exists()"),
    rh_foundation::Invariant::new("exs-19", rh_foundation::Severity::Warning, "Actor should be referenced in at least one operation", "%resource.process.descendants().select(operation).where(initiator=%context.key or receiver=%context.key).exists()"),
    rh_foundation::Invariant::new("exs-2", rh_foundation::Severity::Error, "instance.content is only allowed if there are no instance.versions", "content.exists() implies version.empty()"),
    rh_foundation::Invariant::new("exs-20", rh_foundation::Severity::Warning, "Instance should be referenced in at least one location", "%resource.process.descendants().select(instanceReference).where($this=%context.key).exists()"),
    rh_foundation::Invariant::new("exs-21", rh_foundation::Severity::Warning, "Instance version should be referenced in at least one operation", "version.exists() implies version.key.intersect(%resource.process.descendants().where(instanceReference = %context.key).versionReference).exists()"),
    rh_foundation::Invariant::new("exs-22", rh_foundation::Severity::Error, "Can have a process, a workflow, one or more operations or none of these, but cannot have a combination", "(process.exists() implies workflow.empty() and operation.empty()) and (workflow.exists() implies operation.empty())"),
    rh_foundation::Invariant::new("exs-23", rh_foundation::Severity::Error, "actor.key canot be 'OTHER'", "key != 'OTHER'"),
    rh_foundation::Invariant::new("exs-3", rh_foundation::Severity::Error, "Must have actors if status is active or required", "status='active' or status='retired' implies actor.exists()"),
    rh_foundation::Invariant::new("exs-4", rh_foundation::Severity::Error, "Must have processes if status is active or required", "status='active' or status='retired' implies process.exists()"),
    rh_foundation::Invariant::new("exs-5", rh_foundation::Severity::Error, "Processes must have steps if ExampleScenario status is active or required", "%resource.status='active' or %resource.status='retired' implies step.exists()"),
    rh_foundation::Invariant::new("exs-6", rh_foundation::Severity::Error, "Actor keys must be unique", "actor.key.count() = actor.key.distinct().count()"),
    rh_foundation::Invariant::new("exs-7", rh_foundation::Severity::Error, "Actor titles must be unique", "actor.title.count() = actor.title.distinct().count()"),
    rh_foundation::Invariant::new("exs-8", rh_foundation::Severity::Error, "Instance keys must be unique", "instance.key.count() = instance.key.distinct().count()"),
    rh_foundation::Invariant::new("exs-9", rh_foundation::Severity::Error, "Instance titles must be unique", "instance.title.count() = instance.title.distinct().count()"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()"),
]
    });

/// FHIR required bindings for this resource/datatype
///
/// These bindings define which ValueSets must be used for coded elements.
/// Only 'required' strength bindings are included (extensible/preferred are not enforced).
pub static BINDINGS: once_cell::sync::Lazy<Vec<rh_foundation::ElementBinding>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementBinding::new(
                "ExampleScenario.actor.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/examplescenario-actor-type|5.0.0",
            )
            .with_description("The type of actor - system or human."),
            rh_foundation::ElementBinding::new(
                "ExampleScenario.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ExampleScenario.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/publication-status|5.0.0",
            )
            .with_description("The lifecycle status of an artifact."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ExampleScenario.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.contained", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.extension", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.url", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.version", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.versionAlgorithm[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.name", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.title", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.experimental", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.date", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.publisher", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.contact", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.useContext", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.jurisdiction", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.purpose", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.copyright", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.copyrightLabel", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.actor.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor.key", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.actor.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance.key", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.structureType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.structureVersion",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.structureProfile[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance.content", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.instance.version", 0, None),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.key",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.title",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.version.content",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.containedInstance",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.containedInstance.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.containedInstance.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.containedInstance.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.containedInstance.instanceReference",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.instance.containedInstance.versionReference",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.process", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.process.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ExampleScenario.process.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.process.title", 1, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.preConditions",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.postConditions",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ExampleScenario.process.step", 0, None),
            rh_foundation::ElementCardinality::new("ExampleScenario.process.step.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.number",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.process",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.workflow",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.title",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.initiator",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.receiver",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.initiatorActive",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.receiverActive",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.request",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.operation.response",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative.title",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.alternative.step",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ExampleScenario.process.step.pause",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ExampleScenario {
    fn id(&self) -> Option<String> {
        self.base.base.id.clone()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.base.meta.clone()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.base.implicit_rules.clone()
    }
    fn language(&self) -> Option<String> {
        self.base.base.language.clone()
    }
}

impl crate::traits::resource::ResourceMutators for ExampleScenario {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.id = Some(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base.base.meta = Some(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.implicit_rules = Some(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base.base.language = Some(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for ExampleScenario {
    fn has_id(&self) -> bool {
        self.base.base.id.is_some()
    }
    fn has_meta(&self) -> bool {
        self.base.base.meta.is_some()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.base.implicit_rules.is_some()
    }
    fn has_language(&self) -> bool {
        self.base.base.language.is_some()
    }
}

impl crate::traits::domain_resource::DomainResourceAccessors for ExampleScenario {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_slice()
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_slice()
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_slice()
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for ExampleScenario {
    fn new() -> Self {
        Self::default()
    }
    fn set_text(self, value: crate::datatypes::narrative::Narrative) -> Self {
        let mut resource = self.clone();
        resource.base.text = Some(value);
        resource
    }
    fn set_contained(self, value: Vec<crate::resources::resource::Resource>) -> Self {
        let mut resource = self.clone();
        resource.base.contained = value;
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource.base.contained.push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = value;
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.extension.push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = value;
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension.push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for ExampleScenario {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        !self.base.contained.is_empty()
    }
    fn has_extension(&self) -> bool {
        !self.base.extension.is_empty()
    }
    fn has_modifier_extension(&self) -> bool {
        !self.base.modifier_extension.is_empty()
    }
}

impl crate::traits::example_scenario::ExampleScenarioAccessors for ExampleScenario {
    fn url(&self) -> Option<StringType> {
        self.url.clone()
    }
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn version(&self) -> Option<StringType> {
        self.version.clone()
    }
    fn name(&self) -> Option<StringType> {
        self.name.clone()
    }
    fn title(&self) -> Option<StringType> {
        self.title.clone()
    }
    fn status(&self) -> PublicationStatus {
        self.status.clone()
    }
    fn experimental(&self) -> Option<BooleanType> {
        self.experimental
    }
    fn date(&self) -> Option<DateTimeType> {
        self.date.clone()
    }
    fn publisher(&self) -> Option<StringType> {
        self.publisher.clone()
    }
    fn contact(&self) -> &[ContactDetail] {
        self.contact.as_slice()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn use_context(&self) -> &[UsageContext] {
        self.use_context.as_slice()
    }
    fn jurisdiction(&self) -> &[CodeableConcept] {
        self.jurisdiction.as_slice()
    }
    fn purpose(&self) -> Option<StringType> {
        self.purpose.clone()
    }
    fn copyright(&self) -> Option<StringType> {
        self.copyright.clone()
    }
    fn copyright_label(&self) -> Option<StringType> {
        self.copyright_label.clone()
    }
    fn actor(&self) -> &[ExampleScenarioActor] {
        self.actor.as_slice()
    }
    fn instance(&self) -> &[ExampleScenarioInstance] {
        self.instance.as_slice()
    }
    fn process(&self) -> &[ExampleScenarioProcess] {
        self.process.as_slice()
    }
}

impl crate::traits::example_scenario::ExampleScenarioMutators for ExampleScenario {
    fn new() -> Self {
        Self::default()
    }
    fn set_url(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.url = Some(value);
        resource
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = value;
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.push(item);
        resource
    }
    fn set_version(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.version = Some(value);
        resource
    }
    fn set_name(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.name = Some(value);
        resource
    }
    fn set_title(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.title = Some(value);
        resource
    }
    fn set_status(self, value: PublicationStatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_experimental(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.experimental = Some(value);
        resource
    }
    fn set_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.date = Some(value);
        resource
    }
    fn set_publisher(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.publisher = Some(value);
        resource
    }
    fn set_contact(self, value: Vec<ContactDetail>) -> Self {
        let mut resource = self.clone();
        resource.contact = value;
        resource
    }
    fn add_contact(self, item: ContactDetail) -> Self {
        let mut resource = self.clone();
        resource.contact.push(item);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_use_context(self, value: Vec<UsageContext>) -> Self {
        let mut resource = self.clone();
        resource.use_context = value;
        resource
    }
    fn add_use_context(self, item: UsageContext) -> Self {
        let mut resource = self.clone();
        resource.use_context.push(item);
        resource
    }
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction = value;
        resource
    }
    fn add_jurisdiction(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.jurisdiction.push(item);
        resource
    }
    fn set_purpose(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.purpose = Some(value);
        resource
    }
    fn set_copyright(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright = Some(value);
        resource
    }
    fn set_copyright_label(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.copyright_label = Some(value);
        resource
    }
    fn set_actor(self, value: Vec<ExampleScenarioActor>) -> Self {
        let mut resource = self.clone();
        resource.actor = value;
        resource
    }
    fn add_actor(self, item: ExampleScenarioActor) -> Self {
        let mut resource = self.clone();
        resource.actor.push(item);
        resource
    }
    fn set_instance(self, value: Vec<ExampleScenarioInstance>) -> Self {
        let mut resource = self.clone();
        resource.instance = value;
        resource
    }
    fn add_instance(self, item: ExampleScenarioInstance) -> Self {
        let mut resource = self.clone();
        resource.instance.push(item);
        resource
    }
    fn set_process(self, value: Vec<ExampleScenarioProcess>) -> Self {
        let mut resource = self.clone();
        resource.process = value;
        resource
    }
    fn add_process(self, item: ExampleScenarioProcess) -> Self {
        let mut resource = self.clone();
        resource.process.push(item);
        resource
    }
}

impl crate::traits::example_scenario::ExampleScenarioExistence for ExampleScenario {
    fn has_version_algorithm(&self) -> bool {
        self.version_algorithm_string.is_some() || self.version_algorithm_coding.is_some()
    }
    fn has_url(&self) -> bool {
        self.url.is_some()
    }
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_version(&self) -> bool {
        self.version.is_some()
    }
    fn has_name(&self) -> bool {
        self.name.is_some()
    }
    fn has_title(&self) -> bool {
        self.title.is_some()
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_experimental(&self) -> bool {
        self.experimental.is_some()
    }
    fn has_date(&self) -> bool {
        self.date.is_some()
    }
    fn has_publisher(&self) -> bool {
        self.publisher.is_some()
    }
    fn has_contact(&self) -> bool {
        !self.contact.is_empty()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_use_context(&self) -> bool {
        !self.use_context.is_empty()
    }
    fn has_jurisdiction(&self) -> bool {
        !self.jurisdiction.is_empty()
    }
    fn has_purpose(&self) -> bool {
        self.purpose.is_some()
    }
    fn has_copyright(&self) -> bool {
        self.copyright.is_some()
    }
    fn has_copyright_label(&self) -> bool {
        self.copyright_label.is_some()
    }
    fn has_actor(&self) -> bool {
        !self.actor.is_empty()
    }
    fn has_instance(&self) -> bool {
        !self.instance.is_empty()
    }
    fn has_process(&self) -> bool {
        !self.process.is_empty()
    }
}

impl crate::validation::ValidatableResource for ExampleScenario {
    fn resource_type(&self) -> &'static str {
        "ExampleScenario"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn bindings() -> &'static [rh_foundation::ElementBinding] {
        &BINDINGS
    }

    fn cardinalities() -> &'static [rh_foundation::ElementCardinality] {
        &CARDINALITIES
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/ExampleScenario")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::example_scenario::{
    ExampleScenarioAccessors, ExampleScenarioExistence, ExampleScenarioMutators,
};
