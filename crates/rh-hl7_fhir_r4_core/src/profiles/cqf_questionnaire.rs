use crate::resources::questionnaire::Questionnaire;
use serde::{Deserialize, Serialize};
/// CQF-Questionnaire
///
/// A questionnaire with the ability to specify behavior associated with questions or groups of questions
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/cqf-questionnaire
/// - Version: 4.0.1
/// - Kind: resource
/// - Type: Questionnaire
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/Questionnaire
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CQFQuestionnaire {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: Questionnaire,
}

/// FHIR invariants for this resource/datatype
///
/// These constraints are defined in the FHIR specification and must be validated
/// when creating or modifying instances of this type.
pub static INVARIANTS: once_cell::sync::Lazy<Vec<rh_foundation::Invariant>> =
    once_cell::sync::Lazy::new(|| {
        vec![
    rh_foundation::Invariant::new("dom-2", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL NOT contain nested Resources", "contained.contained.empty()").with_xpath("not(parent::f:contained and f:contained)"),
    rh_foundation::Invariant::new("dom-3", rh_foundation::Severity::Error, "If the resource is contained in another resource, it SHALL be referred to from elsewhere in the resource or SHALL refer to the containing resource", "contained.where((('#'+id in (%resource.descendants().reference | %resource.descendants().as(canonical) | %resource.descendants().as(uri) | %resource.descendants().as(url))) or descendants().where(reference = '#').exists() or descendants().where(as(canonical) = '#').exists() or descendants().where(as(canonical) = '#').exists()).not()).trace('unmatched', id).empty()").with_xpath("not(exists(for $id in f:contained/*/f:id/@value return $contained[not(parent::*/descendant::f:reference/@value=concat('#', $contained/*/id/@value) or descendant::f:reference[@value='#'])]))"),
    rh_foundation::Invariant::new("dom-4", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a meta.versionId or a meta.lastUpdated", "contained.meta.versionId.empty() and contained.meta.lastUpdated.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:versionId)) and not(exists(f:contained/*/f:meta/f:lastUpdated))"),
    rh_foundation::Invariant::new("dom-5", rh_foundation::Severity::Error, "If a resource is contained in another resource, it SHALL NOT have a security label", "contained.meta.security.empty()").with_xpath("not(exists(f:contained/*/f:meta/f:security))"),
    rh_foundation::Invariant::new("dom-6", rh_foundation::Severity::Warning, "A resource should have narrative for robust management", "text.`div`.exists()").with_xpath("exists(f:text/h:div)"),
    rh_foundation::Invariant::new("ele-1", rh_foundation::Severity::Error, "All FHIR elements must have a @value or children", "hasValue() or (children().count() > id.count())").with_xpath("@value|f:*|h:div"),
    rh_foundation::Invariant::new("ext-1", rh_foundation::Severity::Error, "Must have either extensions or value[x], not both", "extension.exists() != value.exists()").with_xpath("exists(f:extension)!=exists(f:*[starts-with(local-name(.), \"value\")])"),
    rh_foundation::Invariant::new("que-0", rh_foundation::Severity::Warning, "Name should be usable as an identifier for the module by machine processing applications such as code generation", "name.matches('[A-Z]([A-Za-z0-9_]){0,254}')").with_xpath("not(exists(f:name/@value)) or matches(f:name/@value, '[A-Z]([A-Za-z0-9_]){0,254}')"),
    rh_foundation::Invariant::new("que-1", rh_foundation::Severity::Error, "Group items must have nested items, display items cannot have nested items", "(type='group' implies item.empty().not()) and (type.trace('type')='display' implies item.trace('item').empty())").with_xpath("not((f:type/@value='group' and not(f:item)) or (f:type/@value='display' and f:item))"),
    rh_foundation::Invariant::new("que-10", rh_foundation::Severity::Error, "Maximum length can only be declared for simple question types", "(type in ('boolean' | 'decimal' | 'integer' | 'string' | 'text' | 'url' | 'open-choice')) or maxLength.empty()").with_xpath("f:type/@value=('boolean', 'decimal', 'integer', 'open-choice', 'string', 'text', 'url') or not(f:maxLength)"),
    rh_foundation::Invariant::new("que-11", rh_foundation::Severity::Error, "If one or more answerOption is present, initial[x] must be missing", "answerOption.empty() or initial.empty()").with_xpath("not(f:answerOption) or not(count(f:*[starts-with(local-name(.), 'initial')]))"),
    rh_foundation::Invariant::new("que-12", rh_foundation::Severity::Error, "If there are more than one enableWhen, enableBehavior must be specified", "enableWhen.count() > 2 implies enableBehavior.exists()").with_xpath("not(f:answerOption) or not(count(f:*[starts-with(local-name(.), 'initial')]))"),
    rh_foundation::Invariant::new("que-13", rh_foundation::Severity::Error, "Can only have multiple initial values for repeating items", "repeats=true or initial.count() <= 1").with_xpath("f:repeats/@value='true' or count(f:initial)<=1"),
    rh_foundation::Invariant::new("que-2", rh_foundation::Severity::Error, "The link ids for groups and questions must be unique within the questionnaire", "descendants().linkId.isDistinct()").with_xpath("count(descendant::f:linkId/@value)=count(distinct-values(descendant::f:linkId/@value))"),
    rh_foundation::Invariant::new("que-3", rh_foundation::Severity::Error, "Display items cannot have a \"code\" asserted", "type!='display' or code.empty()").with_xpath("not(f:type/@value='display' and f:code)"),
    rh_foundation::Invariant::new("que-4", rh_foundation::Severity::Error, "A question cannot have both answerOption and answerValueSet", "answerOption.empty() or answerValueSet.empty()").with_xpath("not(f:answerValueSet and f:answerOption)"),
    rh_foundation::Invariant::new("que-5", rh_foundation::Severity::Error, "Only 'choice' and 'open-choice' items can have answerValueSet", "(type ='choice' or type = 'open-choice' or type = 'decimal' or type = 'integer' or type = 'date' or type = 'dateTime' or type = 'time' or type = 'string' or type = 'quantity') or (answerValueSet.empty() and answerOption.empty())").with_xpath("f:type/@value=('choice','open-choice','decimal','integer','date','dateTime','time','string','quantity',') or not(f:answerOption or f:answerValueSet)"),
    rh_foundation::Invariant::new("que-6", rh_foundation::Severity::Error, "Required and repeat aren't permitted for display items", "type!='display' or (required.empty() and repeats.empty())").with_xpath("not(f:type/@value='display' and (f:required or f:repeats))"),
    rh_foundation::Invariant::new("que-7", rh_foundation::Severity::Error, "If the operator is 'exists', the value must be a boolean", "operator = 'exists' implies (answer is Boolean)").with_xpath("f:operator/@value != 'exists' or exists(f:answerBoolean)"),
    rh_foundation::Invariant::new("que-8", rh_foundation::Severity::Error, "Initial values can't be specified for groups or display items", "(type!='group' and type!='display') or initial.empty()").with_xpath("not(f:type/@value=('group', 'display') and f:*[starts-with(local-name(.), 'initial')])"),
    rh_foundation::Invariant::new("que-9", rh_foundation::Severity::Error, "Read-only can't be specified for \"display\" items", "type!='display' or readOnly.empty()").with_xpath("not(f:type/@value=('group', 'display') and f:*[starts-with(local-name(.), 'initial')])"),
]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for CQFQuestionnaire {
    fn id(&self) -> Option<String> {
        self.base.id()
    }
    fn meta(&self) -> Option<crate::datatypes::meta::Meta> {
        self.base.meta()
    }
    fn implicit_rules(&self) -> Option<String> {
        self.base.implicit_rules()
    }
    fn language(&self) -> Option<String> {
        self.base.language()
    }
}

impl crate::traits::resource::ResourceMutators for CQFQuestionnaire {
    fn new() -> Self {
        Self::default()
    }
    fn set_id(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_id(value);
        resource
    }
    fn set_meta(self, value: crate::datatypes::meta::Meta) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_meta(value);
        resource
    }
    fn set_implicit_rules(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_implicit_rules(value);
        resource
    }
    fn set_language(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.base = resource.base.set_language(value);
        resource
    }
}

impl crate::traits::resource::ResourceExistence for CQFQuestionnaire {
    fn has_id(&self) -> bool {
        self.base.has_id()
    }
    fn has_meta(&self) -> bool {
        self.base.has_meta()
    }
    fn has_implicit_rules(&self) -> bool {
        self.base.has_implicit_rules()
    }
    fn has_language(&self) -> bool {
        self.base.has_language()
    }
}

impl crate::traits::cqf_questionnaire::CQFQuestionnaireMutators for CQFQuestionnaire {
    fn new() -> Self {
        Self::default()
    }
}

impl crate::validation::ValidatableResource for CQFQuestionnaire {
    fn resource_type(&self) -> &'static str {
        "Questionnaire"
    }

    fn invariants() -> &'static [rh_foundation::Invariant] {
        &INVARIANTS
    }

    fn profile_url() -> Option<&'static str> {
        Some("http://hl7.org/fhir/StructureDefinition/cqf-questionnaire")
    }
}
