use crate::bindings::publication_status::PublicationStatus;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::contact_detail::ContactDetail;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::usage_context::UsageContext;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::string::StringType;
use crate::resources::graph_definition::GraphDefinitionLink;
use crate::resources::graph_definition::GraphDefinitionNode;
use crate::traits::domain_resource::DomainResourceAccessors;
use crate::traits::domain_resource::DomainResourceExistence;
use crate::traits::domain_resource::DomainResourceMutators;
/// GraphDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references. The Graph Definition resource defines a set and makes rules about the set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GraphDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GraphDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GraphDefinitionAccessors: DomainResourceAccessors {
    /// Returns a reference to the url field.
    fn url(&self) -> Option<StringType>;
    /// Returns a reference to the identifier field.
    fn identifier(&self) -> &[Identifier];
    /// Returns a reference to the version field.
    fn version(&self) -> Option<StringType>;
    /// Returns a reference to the name field.
    fn name(&self) -> StringType;
    /// Returns a reference to the title field.
    fn title(&self) -> Option<StringType>;
    /// Returns a reference to the status field.
    fn status(&self) -> PublicationStatus;
    /// Returns a reference to the experimental field.
    fn experimental(&self) -> Option<BooleanType>;
    /// Returns a reference to the date field.
    fn date(&self) -> Option<DateTimeType>;
    /// Returns a reference to the publisher field.
    fn publisher(&self) -> Option<StringType>;
    /// Returns a reference to the contact field.
    fn contact(&self) -> &[ContactDetail];
    /// Returns a reference to the description field.
    fn description(&self) -> Option<StringType>;
    /// Returns a reference to the useContext field.
    fn use_context(&self) -> &[UsageContext];
    /// Returns a reference to the jurisdiction field.
    fn jurisdiction(&self) -> &[CodeableConcept];
    /// Returns a reference to the purpose field.
    fn purpose(&self) -> Option<StringType>;
    /// Returns a reference to the copyright field.
    fn copyright(&self) -> Option<StringType>;
    /// Returns a reference to the copyrightLabel field.
    fn copyright_label(&self) -> Option<StringType>;
    /// Returns a reference to the start field.
    fn start(&self) -> Option<StringType>;
    /// Returns a reference to the node field.
    fn node(&self) -> &[GraphDefinitionNode];
    /// Returns a reference to the link field.
    fn link(&self) -> &[GraphDefinitionLink];
}
/// GraphDefinition Trait
///
/// This trait provides common functionality and access patterns for this FHIR resource type.
///
/// A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references. The Graph Definition resource defines a set and makes rules about the set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GraphDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GraphDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GraphDefinitionMutators: DomainResourceMutators {
    /// Create a new GraphDefinition with default/empty values.
    ///
    /// All optional fields will be set to None and array fields will be empty vectors.
    /// Supports method chaining with set_xxx() and add_xxx() methods.
    ///
    /// # Example
    /// ```rust
    /// use rh_hl7_fhir_r5_core::resources::graph_definition::GraphDefinition;
    /// use rh_hl7_fhir_r5_core::traits::graph_definition::GraphDefinitionMutators;
    ///
    /// let resource = GraphDefinition::new();
    /// // Can be used with method chaining:
    /// // resource.set_field(value).add_item(item);
    /// ```
    fn new() -> Self;
    /// Sets the url field and returns self for chaining.
    fn set_url(self, value: String) -> Self;
    /// Sets the identifier field and returns self for chaining.
    fn set_identifier(self, value: Vec<Identifier>) -> Self;
    /// Adds an item to the identifier field and returns self for chaining.
    fn add_identifier(self, item: Identifier) -> Self;
    /// Sets the version field and returns self for chaining.
    fn set_version(self, value: String) -> Self;
    /// Sets the name field and returns self for chaining.
    fn set_name(self, value: String) -> Self;
    /// Sets the title field and returns self for chaining.
    fn set_title(self, value: String) -> Self;
    /// Sets the status field and returns self for chaining.
    fn set_status(self, value: PublicationStatus) -> Self;
    /// Sets the experimental field and returns self for chaining.
    fn set_experimental(self, value: bool) -> Self;
    /// Sets the date field and returns self for chaining.
    fn set_date(self, value: String) -> Self;
    /// Sets the publisher field and returns self for chaining.
    fn set_publisher(self, value: String) -> Self;
    /// Sets the contact field and returns self for chaining.
    fn set_contact(self, value: Vec<ContactDetail>) -> Self;
    /// Adds an item to the contact field and returns self for chaining.
    fn add_contact(self, item: ContactDetail) -> Self;
    /// Sets the description field and returns self for chaining.
    fn set_description(self, value: String) -> Self;
    /// Sets the useContext field and returns self for chaining.
    fn set_use_context(self, value: Vec<UsageContext>) -> Self;
    /// Adds an item to the useContext field and returns self for chaining.
    fn add_use_context(self, item: UsageContext) -> Self;
    /// Sets the jurisdiction field and returns self for chaining.
    fn set_jurisdiction(self, value: Vec<CodeableConcept>) -> Self;
    /// Adds an item to the jurisdiction field and returns self for chaining.
    fn add_jurisdiction(self, item: CodeableConcept) -> Self;
    /// Sets the purpose field and returns self for chaining.
    fn set_purpose(self, value: String) -> Self;
    /// Sets the copyright field and returns self for chaining.
    fn set_copyright(self, value: String) -> Self;
    /// Sets the copyrightLabel field and returns self for chaining.
    fn set_copyright_label(self, value: String) -> Self;
    /// Sets the start field and returns self for chaining.
    fn set_start(self, value: String) -> Self;
    /// Sets the node field and returns self for chaining.
    fn set_node(self, value: Vec<GraphDefinitionNode>) -> Self;
    /// Adds an item to the node field and returns self for chaining.
    fn add_node(self, item: GraphDefinitionNode) -> Self;
    /// Sets the link field and returns self for chaining.
    fn set_link(self, value: Vec<GraphDefinitionLink>) -> Self;
    /// Adds an item to the link field and returns self for chaining.
    fn add_link(self, item: GraphDefinitionLink) -> Self;
}
/// GraphDefinition Existence Checks
///
/// This trait provides existence check methods for this FHIR resource type.
///
/// A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references. The Graph Definition resource defines a set and makes rules about the set.
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/GraphDefinition
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: GraphDefinition
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
pub trait GraphDefinitionExistence: DomainResourceExistence {
    /// Returns true if the url field is present (Some).
    fn has_url(&self) -> bool;
    /// Returns true if the identifier field is not empty.
    fn has_identifier(&self) -> bool;
    /// Returns true if the version field is present (Some).
    fn has_version(&self) -> bool;
    /// Returns true if the version_algorithm field is present (Some).
    fn has_version_algorithm(&self) -> bool;
    /// Returns true if the name field is present (Some).
    fn has_name(&self) -> bool;
    /// Returns true if the title field is present (Some).
    fn has_title(&self) -> bool;
    /// Returns true if the status field is present (Some).
    fn has_status(&self) -> bool;
    /// Returns true if the experimental field is present (Some).
    fn has_experimental(&self) -> bool;
    /// Returns true if the date field is present (Some).
    fn has_date(&self) -> bool;
    /// Returns true if the publisher field is present (Some).
    fn has_publisher(&self) -> bool;
    /// Returns true if the contact field is not empty.
    fn has_contact(&self) -> bool;
    /// Returns true if the description field is present (Some).
    fn has_description(&self) -> bool;
    /// Returns true if the use_context field is not empty.
    fn has_use_context(&self) -> bool;
    /// Returns true if the jurisdiction field is not empty.
    fn has_jurisdiction(&self) -> bool;
    /// Returns true if the purpose field is present (Some).
    fn has_purpose(&self) -> bool;
    /// Returns true if the copyright field is present (Some).
    fn has_copyright(&self) -> bool;
    /// Returns true if the copyright_label field is present (Some).
    fn has_copyright_label(&self) -> bool;
    /// Returns true if the start field is present (Some).
    fn has_start(&self) -> bool;
    /// Returns true if the node field is not empty.
    fn has_node(&self) -> bool;
    /// Returns true if the link field is not empty.
    fn has_link(&self) -> bool;
}
