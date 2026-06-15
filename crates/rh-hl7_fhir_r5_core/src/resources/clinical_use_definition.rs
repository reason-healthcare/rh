use crate::bindings::clinical_use_definition_type::ClinicalUseDefinitionType;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::element::Element;
use crate::datatypes::expression::Expression;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::range::Range;
use crate::datatypes::reference::Reference;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// ClinicalUseDefinition
///
/// A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/ClinicalUseDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: ClinicalUseDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinition {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// Business identifier for this issue
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identifier: Vec<Identifier>,
    /// indication | contraindication | interaction | undesirable-effect | warning
    #[serde(rename = "type")]
    pub type_: ClinicalUseDefinitionType,
    /// Extension element for the 'type' primitive field. Contains metadata and extensions.
    pub _type: Option<Element>,
    /// A categorisation of the issue, primarily for dividing warnings into subject heading areas such as "Pregnancy", "Overdose"
    ///
    /// Binding: preferred (A categorisation for a clinical use information item.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/clinical-use-definition-category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<CodeableConcept>,
    /// The medication, product, substance, device, procedure etc. for which this is an indication
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subject: Vec<Reference>,
    /// Whether this is a current issue or one that has been retired etc
    ///
    /// Binding: preferred (The lifecycle status of an artifact.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/publication-status
    pub status: Option<CodeableConcept>,
    /// Specifics for when this is a contraindication
    pub contraindication: Option<ClinicalUseDefinitionContraindication>,
    /// Specifics for when this is an indication
    pub indication: Option<ClinicalUseDefinitionIndication>,
    /// Specifics for when this is an interaction
    pub interaction: Option<ClinicalUseDefinitionInteraction>,
    /// The population group to which this applies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub population: Vec<Reference>,
    /// Logic used by the clinical use definition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub library: Vec<StringType>,
    /// Extension element for the 'library' primitive field. Contains metadata and extensions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub _library: Vec<Element>,
    /// A possible negative outcome from the use of this treatment
    #[serde(rename = "undesirableEffect")]
    pub undesirable_effect: Option<ClinicalUseDefinitionUndesirableeffect>,
    /// Critical environmental, health or physical risks or hazards. For example 'Do not operate heavy machinery', 'May cause drowsiness'
    pub warning: Option<ClinicalUseDefinitionWarning>,
}
/// ClinicalUseDefinition nested structure for the 'contraindication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionContraindication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Information about use of the product in relation to other therapies described as part of the contraindication
    #[serde(rename = "otherTherapy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_therapy: Vec<ClinicalUseDefinitionContraindicationOthertherapy>,
    /// The situation that is being documented as contraindicating against this item
    ///
    /// Binding: example (A symptom, disease or procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/disease-symptom-procedure
    #[serde(rename = "diseaseSymptomProcedure")]
    pub disease_symptom_procedure: Option<CodeableReference>,
    /// The status of the disease or symptom for the contraindication
    ///
    /// Binding: example (The status of a disease or symptom.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/disease-status
    #[serde(rename = "diseaseStatus")]
    pub disease_status: Option<CodeableReference>,
    /// A comorbidity (concurrent condition) or coinfection
    ///
    /// Binding: example (A symptom, disease or procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/disease-symptom-procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comorbidity: Vec<CodeableReference>,
    /// The indication which this is a contraidication for
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indication: Vec<Reference>,
    /// An expression that returns true or false, indicating whether the indication is applicable or not, after having applied its other elements
    pub applicability: Option<Expression>,
}
/// ClinicalUseDefinitionContraindication nested structure for the 'otherTherapy' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionContraindicationOthertherapy {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The type of relationship between the product indication/contraindication and another therapy
    ///
    /// Binding: preferred (Classification of relationship between a therapy and a contraindication or an indication.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/therapy-relationship-type
    #[serde(rename = "relationshipType")]
    pub relationship_type: CodeableConcept,
    /// Reference to a specific medication, substance etc. as part of an indication or contraindication
    ///
    /// Binding: example (A therapy.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/therapy
    pub treatment: CodeableReference,
}
/// ClinicalUseDefinition nested structure for the 'indication' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionIndication {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The situation that is being documented as an indicaton for this item
    ///
    /// Binding: example (A symptom, disease or procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/disease-symptom-procedure
    #[serde(rename = "diseaseSymptomProcedure")]
    pub disease_symptom_procedure: Option<CodeableReference>,
    /// The status of the disease or symptom for the indication
    ///
    /// Binding: example (The status of a disease or symptom.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/disease-status
    #[serde(rename = "diseaseStatus")]
    pub disease_status: Option<CodeableReference>,
    /// A comorbidity or coinfection as part of the indication
    ///
    /// Binding: example (A symptom, disease or procedure.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/disease-symptom-procedure
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comorbidity: Vec<CodeableReference>,
    /// The intended effect, aim or strategy to be achieved
    ///
    /// Binding: preferred (The overall intended use of a product.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/product-intended-use
    #[serde(rename = "intendedEffect")]
    pub intended_effect: Option<CodeableReference>,
    /// Timing or duration information (Range)
    #[serde(rename = "durationRange")]
    pub duration_range: Option<Range>,
    /// Timing or duration information (string)
    #[serde(rename = "durationString")]
    pub duration_string: Option<StringType>,
    /// An unwanted side effect or negative outcome of the subject of this resource when being used for this indication
    #[serde(rename = "undesirableEffect")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub undesirable_effect: Vec<Reference>,
    /// An expression that returns true or false, indicating whether the indication is applicable or not, after having applied its other elements
    pub applicability: Option<Expression>,
    /// The use of the medicinal product in relation to other therapies described as part of the indication
    #[serde(rename = "otherTherapy")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_therapy: Vec<StringType>,
}
/// ClinicalUseDefinition nested structure for the 'interaction' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionInteraction {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The specific medication, product, food etc. or laboratory test that interacts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interactant: Vec<ClinicalUseDefinitionInteractionInteractant>,
    /// The type of the interaction e.g. drug-drug interaction, drug-lab test interaction
    ///
    /// Binding: example (A categorisation for an interaction between two substances.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/interaction-type
    #[serde(rename = "type")]
    pub type_: Option<CodeableConcept>,
    /// The effect of the interaction, for example "reduced gastric absorption of primary medication"
    ///
    /// Binding: example (A interaction effect of clinical use of a medication or other substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/interaction-effect
    pub effect: Option<CodeableReference>,
    /// The incidence of the interaction, e.g. theoretical, observed
    ///
    /// Binding: example (A categorisation for incidence of occurence of an interaction.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/interaction-incidence
    pub incidence: Option<CodeableConcept>,
    /// Actions for managing the interaction
    ///
    /// Binding: example (A type of management for an interaction of a medication or other substance.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/interaction-management
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub management: Vec<CodeableConcept>,
}
/// ClinicalUseDefinitionInteraction nested structure for the 'interactant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionInteractionInteractant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The specific medication, product, food etc. or laboratory test that interacts (Reference)
    #[serde(rename = "itemReference")]
    pub item_reference: Reference,
    /// The specific medication, product, food etc. or laboratory test that interacts (CodeableConcept)
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
}
/// ClinicalUseDefinition nested structure for the 'undesirableEffect' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionUndesirableeffect {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// The situation in which the undesirable effect may manifest
    ///
    /// Binding: example (An undesirable effect of clinical use.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/undesirable-effect-symptom
    #[serde(rename = "symptomConditionEffect")]
    pub symptom_condition_effect: Option<CodeableReference>,
    /// High level classification of the effect
    ///
    /// Binding: example (A categorisation for an undesirable effect.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/undesirable-effect-classification
    pub classification: Option<CodeableConcept>,
    /// How often the effect is seen
    ///
    /// Binding: example (A categorisation for a frequency of occurence of an undesirable effect.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/undesirable-effect-frequency
    #[serde(rename = "frequencyOfOccurrence")]
    pub frequency_of_occurrence: Option<CodeableConcept>,
}
/// ClinicalUseDefinition nested structure for the 'warning' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalUseDefinitionWarning {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// A textual definition of this warning, with formatting
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// A coded or unformatted textual definition of this warning
    ///
    /// Binding: example (Classification of warning type.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/warning-type
    pub code: Option<CodeableConcept>,
}

impl Default for ClinicalUseDefinition {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            type_: Default::default(),
            _type: Default::default(),
            category: Default::default(),
            subject: Default::default(),
            status: Default::default(),
            contraindication: Default::default(),
            indication: Default::default(),
            interaction: Default::default(),
            population: Default::default(),
            library: Default::default(),
            _library: Default::default(),
            undesirable_effect: Default::default(),
            warning: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionContraindication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            other_therapy: Default::default(),
            disease_symptom_procedure: Default::default(),
            disease_status: Default::default(),
            comorbidity: Default::default(),
            indication: Default::default(),
            applicability: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionContraindicationOthertherapy {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            relationship_type: Default::default(),
            treatment: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionIndication {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            disease_symptom_procedure: Default::default(),
            disease_status: Default::default(),
            comorbidity: Default::default(),
            intended_effect: Default::default(),
            duration_range: Default::default(),
            duration_string: Default::default(),
            undesirable_effect: Default::default(),
            applicability: Default::default(),
            other_therapy: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionInteraction {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            interactant: Default::default(),
            type_: Default::default(),
            effect: Default::default(),
            incidence: Default::default(),
            management: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionInteractionInteractant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            item_reference: Default::default(),
            item_codeable_concept: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionUndesirableeffect {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            symptom_condition_effect: Default::default(),
            classification: Default::default(),
            frequency_of_occurrence: Default::default(),
        }
    }
}

impl Default for ClinicalUseDefinitionWarning {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            description: Default::default(),
            _description: Default::default(),
            code: Default::default(),
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
    rh_foundation::Invariant::new("cud-1", rh_foundation::Severity::Error, "Indication, Contraindication, Interaction, UndesirableEffect and Warning cannot be used in the same instance", "(ClinicalUseDefinition.indication.count() + ClinicalUseDefinition.contraindication.count() + ClinicalUseDefinition.interaction.count() + ClinicalUseDefinition.undesirableEffect.count() + ClinicalUseDefinition.warning.count())  < 2"),
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().ofType(canonical) | %resource.descendants().ofType(uri) | %resource.descendants().ofType(url))) or descendants().where(reference = '#').exists() or descendants().where(ofType(canonical) = '#').exists() or descendants().where(ofType(canonical) = '#').exists()).not()).trace('unmatched', id).empty()"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())"),
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
                "ClinicalUseDefinition.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "ClinicalUseDefinition.type",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/clinical-use-definition-type|5.0.0",
            )
            .with_description("Overall defining type of this clinical use definition."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.implicitRules",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.contained", 0, None),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.identifier", 0, None),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.type", 1, Some(1)),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.category", 0, None),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.subject", 0, None),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.status", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.diseaseSymptomProcedure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.diseaseStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.comorbidity",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.indication",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.applicability",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.otherTherapy",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.otherTherapy.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.otherTherapy.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.otherTherapy.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.otherTherapy.relationshipType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.contraindication.otherTherapy.treatment",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.indication", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.diseaseSymptomProcedure",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.diseaseStatus",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.comorbidity",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.intendedEffect",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.duration[x]",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.undesirableEffect",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.applicability",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.indication.otherTherapy",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.interaction", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.interactant",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.interactant.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.interactant.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.interactant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.interactant.item[x]",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.type",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.effect",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.incidence",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.interaction.management",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.population", 0, None),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.library", 0, None),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect.symptomConditionEffect",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect.classification",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.undesirableEffect.frequencyOfOccurrence",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.warning", 0, Some(1)),
            rh_foundation::ElementCardinality::new("ClinicalUseDefinition.warning.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.warning.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.warning.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.warning.description",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "ClinicalUseDefinition.warning.code",
                0,
                Some(1),
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for ClinicalUseDefinition {
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

impl crate::traits::resource::ResourceMutators for ClinicalUseDefinition {
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

impl crate::traits::resource::ResourceExistence for ClinicalUseDefinition {
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

impl crate::traits::domain_resource::DomainResourceAccessors for ClinicalUseDefinition {
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

impl crate::traits::domain_resource::DomainResourceMutators for ClinicalUseDefinition {
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

impl crate::traits::domain_resource::DomainResourceExistence for ClinicalUseDefinition {
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

impl crate::traits::clinical_use_definition::ClinicalUseDefinitionAccessors
    for ClinicalUseDefinition
{
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_slice()
    }
    fn type_(&self) -> ClinicalUseDefinitionType {
        self.type_.clone()
    }
    fn category(&self) -> &[CodeableConcept] {
        self.category.as_slice()
    }
    fn subject(&self) -> &[Reference] {
        self.subject.as_slice()
    }
    fn status(&self) -> Option<CodeableConcept> {
        self.status.clone()
    }
    fn contraindication(&self) -> Option<ClinicalUseDefinitionContraindication> {
        self.contraindication.clone()
    }
    fn indication(&self) -> Option<ClinicalUseDefinitionIndication> {
        self.indication.clone()
    }
    fn interaction(&self) -> Option<ClinicalUseDefinitionInteraction> {
        self.interaction.clone()
    }
    fn population(&self) -> &[Reference] {
        self.population.as_slice()
    }
    fn library(&self) -> &[StringType] {
        self.library.as_slice()
    }
    fn undesirable_effect(&self) -> Option<ClinicalUseDefinitionUndesirableeffect> {
        self.undesirable_effect.clone()
    }
    fn warning(&self) -> Option<ClinicalUseDefinitionWarning> {
        self.warning.clone()
    }
}

impl crate::traits::clinical_use_definition::ClinicalUseDefinitionMutators
    for ClinicalUseDefinition
{
    fn new() -> Self {
        Self::default()
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
    fn set_type_(self, value: ClinicalUseDefinitionType) -> Self {
        let mut resource = self.clone();
        resource.type_ = value;
        resource
    }
    fn set_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.category = value;
        resource
    }
    fn add_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.category.push(item);
        resource
    }
    fn set_subject(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.subject = value;
        resource
    }
    fn add_subject(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject.push(item);
        resource
    }
    fn set_status(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.status = Some(value);
        resource
    }
    fn set_contraindication(self, value: ClinicalUseDefinitionContraindication) -> Self {
        let mut resource = self.clone();
        resource.contraindication = Some(value);
        resource
    }
    fn set_indication(self, value: ClinicalUseDefinitionIndication) -> Self {
        let mut resource = self.clone();
        resource.indication = Some(value);
        resource
    }
    fn set_interaction(self, value: ClinicalUseDefinitionInteraction) -> Self {
        let mut resource = self.clone();
        resource.interaction = Some(value);
        resource
    }
    fn set_population(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.population = value;
        resource
    }
    fn add_population(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.population.push(item);
        resource
    }
    fn set_library(self, value: Vec<String>) -> Self {
        let mut resource = self.clone();
        resource.library = value;
        resource
    }
    fn add_library(self, item: String) -> Self {
        let mut resource = self.clone();
        resource.library.push(item);
        resource
    }
    fn set_undesirable_effect(self, value: ClinicalUseDefinitionUndesirableeffect) -> Self {
        let mut resource = self.clone();
        resource.undesirable_effect = Some(value);
        resource
    }
    fn set_warning(self, value: ClinicalUseDefinitionWarning) -> Self {
        let mut resource = self.clone();
        resource.warning = Some(value);
        resource
    }
}

impl crate::traits::clinical_use_definition::ClinicalUseDefinitionExistence
    for ClinicalUseDefinition
{
    fn has_identifier(&self) -> bool {
        !self.identifier.is_empty()
    }
    fn has_type_(&self) -> bool {
        true
    }
    fn has_category(&self) -> bool {
        !self.category.is_empty()
    }
    fn has_subject(&self) -> bool {
        !self.subject.is_empty()
    }
    fn has_status(&self) -> bool {
        self.status.is_some()
    }
    fn has_contraindication(&self) -> bool {
        self.contraindication.is_some()
    }
    fn has_indication(&self) -> bool {
        self.indication.is_some()
    }
    fn has_interaction(&self) -> bool {
        self.interaction.is_some()
    }
    fn has_population(&self) -> bool {
        !self.population.is_empty()
    }
    fn has_library(&self) -> bool {
        !self.library.is_empty()
    }
    fn has_undesirable_effect(&self) -> bool {
        self.undesirable_effect.is_some()
    }
    fn has_warning(&self) -> bool {
        self.warning.is_some()
    }
}

impl crate::validation::ValidatableResource for ClinicalUseDefinition {
    fn resource_type(&self) -> &'static str {
        "ClinicalUseDefinition"
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
        Some("http://hl7.org/fhir/StructureDefinition/ClinicalUseDefinition")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::clinical_use_definition::{
    ClinicalUseDefinitionAccessors, ClinicalUseDefinitionExistence, ClinicalUseDefinitionMutators,
};
