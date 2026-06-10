use crate::bindings::appointmentstatus::Appointmentstatus;
use crate::bindings::participationstatus::Participationstatus;
use crate::datatypes::annotation::Annotation;
use crate::datatypes::backbone_element::BackboneElement;
use crate::datatypes::codeable_concept::CodeableConcept;
use crate::datatypes::codeable_reference::CodeableReference;
use crate::datatypes::coding::Coding;
use crate::datatypes::element::Element;
use crate::datatypes::identifier::Identifier;
use crate::datatypes::period::Period;
use crate::datatypes::reference::Reference;
use crate::datatypes::virtual_service_detail::VirtualServiceDetail;
use crate::primitives::boolean::BooleanType;
use crate::primitives::date::DateType;
use crate::primitives::date_time::DateTimeType;
use crate::primitives::instant::InstantType;
use crate::primitives::positive_int::PositiveIntType;
use crate::primitives::string::StringType;
use crate::resources::domain_resource::DomainResource;
use serde::{Deserialize, Serialize};
/// Appointment
///
/// A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s).
///
/// **Source:**
/// - URL: http://hl7.org/fhir/StructureDefinition/Appointment
/// - Version: 5.0.0
/// - Kind: resource
/// - Type: Appointment
/// - Base Definition: http://hl7.org/fhir/StructureDefinition/DomainResource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: DomainResource,
    /// External Ids for this item
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | pending | booked | arrived | fulfilled | cancelled | noshow | entered-in-error | checked-in | waitlist
    pub status: Appointmentstatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
    /// The coded reason for the appointment being cancelled
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/appointment-cancellation-reason
    #[serde(rename = "cancellationReason")]
    pub cancellation_reason: Option<CodeableConcept>,
    /// Classification when becoming an encounter
    ///
    /// Binding: preferred (Classification of the encounter.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/EncounterClass
    pub class: Option<Vec<CodeableConcept>>,
    /// A broad categorization of the service that is to be performed during this appointment
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-category
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,
    /// The specific service that is to be performed during this appointment
    ///
    /// Binding: example (No description)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/service-type
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableReference>>,
    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    ///
    /// Binding: preferred (No description)
    ///
    /// Available values:
    /// - `408467006`: Adult mental illness
    /// - `394577000`: Anesthetics
    /// - `394578005`: Audiological medicine
    /// - `421661004`: Blood banking and transfusion medicine
    /// - `408462000`: Burns care
    /// - `394579002`: Cardiology
    /// - `394804000`: Clinical cytogenetics and molecular genetics
    /// - `394580004`: Clinical genetics
    /// - `394803006`: Clinical hematology
    /// - `408480009`: Clinical immunology
    /// - ... and 107 more values
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The style of appointment or patient that has been booked in the slot (not service type)
    ///
    /// Binding: preferred (No description)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v2-0276
    #[serde(rename = "appointmentType")]
    pub appointment_type: Option<CodeableConcept>,
    /// Reason this appointment is scheduled
    ///
    /// Binding: preferred (The Reason for the appointment to take place.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-reason
    pub reason: Option<Vec<CodeableReference>>,
    /// Used to make informed decisions if needing to re-prioritize
    ///
    /// Binding: example (Indicates the urgency of the appointment.)
    ///
    /// ValueSet: http://terminology.hl7.org/ValueSet/v3-ActPriority
    pub priority: Option<CodeableConcept>,
    /// Shown on a subject line in a meeting request, or appointment list
    pub description: Option<StringType>,
    /// Extension element for the 'description' primitive field. Contains metadata and extensions.
    pub _description: Option<Element>,
    /// Appointment replaced by this Appointment
    pub replaces: Option<Vec<Reference>>,
    /// Connection details of a virtual service (e.g. conference call)
    #[serde(rename = "virtualService")]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,
    /// Additional information to support the appointment
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Reference>>,
    /// The previous appointment in a series
    #[serde(rename = "previousAppointment")]
    pub previous_appointment: Option<Reference>,
    /// The originating appointment in a recurring set of appointments
    #[serde(rename = "originatingAppointment")]
    pub originating_appointment: Option<Reference>,
    /// When appointment is to take place
    pub start: Option<InstantType>,
    /// Extension element for the 'start' primitive field. Contains metadata and extensions.
    pub _start: Option<Element>,
    /// When appointment is to conclude
    pub end: Option<InstantType>,
    /// Extension element for the 'end' primitive field. Contains metadata and extensions.
    pub _end: Option<Element>,
    /// Can be less than start/end (e.g. estimate)
    #[serde(rename = "minutesDuration")]
    pub minutes_duration: Option<PositiveIntType>,
    /// Extension element for the 'minutesDuration' primitive field. Contains metadata and extensions.
    #[serde(rename = "_minutesDuration")]
    pub _minutes_duration: Option<Element>,
    /// Potential date/time interval(s) requested to allocate the appointment within
    #[serde(rename = "requestedPeriod")]
    pub requested_period: Option<Vec<Period>>,
    /// The slots that this appointment is filling
    pub slot: Option<Vec<Reference>>,
    /// The set of accounts that may be used for billing for this Appointment
    pub account: Option<Vec<Reference>>,
    /// The date that this appointment was initially created
    pub created: Option<DateTimeType>,
    /// Extension element for the 'created' primitive field. Contains metadata and extensions.
    pub _created: Option<Element>,
    /// When the appointment was cancelled
    #[serde(rename = "cancellationDate")]
    pub cancellation_date: Option<DateTimeType>,
    /// Extension element for the 'cancellationDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_cancellationDate")]
    pub _cancellation_date: Option<Element>,
    /// Additional comments
    pub note: Option<Vec<Annotation>>,
    /// Detailed information and instructions for the patient
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<Vec<CodeableReference>>,
    /// The request this appointment is allocated to assess
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Reference>>,
    /// The patient or group associated with the appointment
    pub subject: Option<Reference>,
    /// Participants involved in appointment
    pub participant: Vec<AppointmentParticipant>,
    /// The sequence number in the recurrence
    #[serde(rename = "recurrenceId")]
    pub recurrence_id: Option<PositiveIntType>,
    /// Extension element for the 'recurrenceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_recurrenceId")]
    pub _recurrence_id: Option<Element>,
    /// Indicates that this appointment varies from a recurrence pattern
    #[serde(rename = "occurrenceChanged")]
    pub occurrence_changed: Option<BooleanType>,
    /// Extension element for the 'occurrenceChanged' primitive field. Contains metadata and extensions.
    #[serde(rename = "_occurrenceChanged")]
    pub _occurrence_changed: Option<Element>,
    /// Details of the recurrence pattern/template used to generate occurrences
    #[serde(rename = "recurrenceTemplate")]
    pub recurrence_template: Option<Vec<AppointmentRecurrencetemplate>>,
}
/// AppointmentRecurrencetemplate nested structure for the 'yearlyTemplate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRecurrencetemplateYearlytemplate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Recurs every nth year
    #[serde(rename = "yearInterval")]
    pub year_interval: PositiveIntType,
    /// Extension element for the 'yearInterval' primitive field. Contains metadata and extensions.
    #[serde(rename = "_yearInterval")]
    pub _year_interval: Option<Element>,
}
/// Appointment nested structure for the 'recurrenceTemplate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRecurrencetemplate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Information about weekly recurring appointments
    #[serde(rename = "weeklyTemplate")]
    pub weekly_template: Option<AppointmentRecurrencetemplateWeeklytemplate>,
    /// Information about yearly recurring appointments
    #[serde(rename = "yearlyTemplate")]
    pub yearly_template: Option<AppointmentRecurrencetemplateYearlytemplate>,
    /// Information about monthly recurring appointments
    #[serde(rename = "monthlyTemplate")]
    pub monthly_template: Option<AppointmentRecurrencetemplateMonthlytemplate>,
    /// The timezone of the occurrences
    pub timezone: Option<CodeableConcept>,
    /// The frequency of the recurrence
    ///
    /// Binding: preferred (IANA Timezones (BCP 175))
    ///
    /// Available values:
    /// - `d`: day
    /// - `wk`: week
    /// - `mo`: month
    /// - `a`: year
    #[serde(rename = "recurrenceType")]
    pub recurrence_type: CodeableConcept,
    /// The date when the recurrence should end
    #[serde(rename = "lastOccurrenceDate")]
    pub last_occurrence_date: Option<DateType>,
    /// Extension element for the 'lastOccurrenceDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_lastOccurrenceDate")]
    pub _last_occurrence_date: Option<Element>,
    /// The number of planned occurrences
    #[serde(rename = "occurrenceCount")]
    pub occurrence_count: Option<PositiveIntType>,
    /// Extension element for the 'occurrenceCount' primitive field. Contains metadata and extensions.
    #[serde(rename = "_occurrenceCount")]
    pub _occurrence_count: Option<Element>,
    /// Specific dates for a recurring set of appointments (no template)
    #[serde(rename = "occurrenceDate")]
    pub occurrence_date: Option<Vec<DateType>>,
    /// Extension element for the 'occurrenceDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_occurrenceDate")]
    pub _occurrence_date: Option<Element>,
    /// Any dates that should be excluded from the series
    #[serde(rename = "excludingDate")]
    pub excluding_date: Option<Vec<DateType>>,
    /// Extension element for the 'excludingDate' primitive field. Contains metadata and extensions.
    #[serde(rename = "_excludingDate")]
    pub _excluding_date: Option<Element>,
    /// Any recurrence IDs that should be excluded from the recurrence
    #[serde(rename = "excludingRecurrenceId")]
    pub excluding_recurrence_id: Option<Vec<PositiveIntType>>,
    /// Extension element for the 'excludingRecurrenceId' primitive field. Contains metadata and extensions.
    #[serde(rename = "_excludingRecurrenceId")]
    pub _excluding_recurrence_id: Option<Element>,
}
/// AppointmentRecurrencetemplate nested structure for the 'weeklyTemplate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRecurrencetemplateWeeklytemplate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Recurs on Mondays
    pub monday: Option<BooleanType>,
    /// Extension element for the 'monday' primitive field. Contains metadata and extensions.
    pub _monday: Option<Element>,
    /// Recurs on Tuesday
    pub tuesday: Option<BooleanType>,
    /// Extension element for the 'tuesday' primitive field. Contains metadata and extensions.
    pub _tuesday: Option<Element>,
    /// Recurs on Wednesday
    pub wednesday: Option<BooleanType>,
    /// Extension element for the 'wednesday' primitive field. Contains metadata and extensions.
    pub _wednesday: Option<Element>,
    /// Recurs on Thursday
    pub thursday: Option<BooleanType>,
    /// Extension element for the 'thursday' primitive field. Contains metadata and extensions.
    pub _thursday: Option<Element>,
    /// Recurs on Friday
    pub friday: Option<BooleanType>,
    /// Extension element for the 'friday' primitive field. Contains metadata and extensions.
    pub _friday: Option<Element>,
    /// Recurs on Saturday
    pub saturday: Option<BooleanType>,
    /// Extension element for the 'saturday' primitive field. Contains metadata and extensions.
    pub _saturday: Option<Element>,
    /// Recurs on Sunday
    pub sunday: Option<BooleanType>,
    /// Extension element for the 'sunday' primitive field. Contains metadata and extensions.
    pub _sunday: Option<Element>,
    /// Recurs every nth week
    #[serde(rename = "weekInterval")]
    pub week_interval: Option<PositiveIntType>,
    /// Extension element for the 'weekInterval' primitive field. Contains metadata and extensions.
    #[serde(rename = "_weekInterval")]
    pub _week_interval: Option<Element>,
}
/// AppointmentRecurrencetemplate nested structure for the 'monthlyTemplate' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentRecurrencetemplateMonthlytemplate {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Recurs on a specific day of the month
    #[serde(rename = "dayOfMonth")]
    pub day_of_month: Option<PositiveIntType>,
    /// Extension element for the 'dayOfMonth' primitive field. Contains metadata and extensions.
    #[serde(rename = "_dayOfMonth")]
    pub _day_of_month: Option<Element>,
    /// Indicates which week of the month the appointment should occur
    #[serde(rename = "nthWeekOfMonth")]
    pub nth_week_of_month: Option<Coding>,
    /// Indicates which day of the week the appointment should occur
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: Option<Coding>,
    /// Recurs every nth month
    #[serde(rename = "monthInterval")]
    pub month_interval: PositiveIntType,
    /// Extension element for the 'monthInterval' primitive field. Contains metadata and extensions.
    #[serde(rename = "_monthInterval")]
    pub _month_interval: Option<Element>,
}
/// Appointment nested structure for the 'participant' field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentParticipant {
    /// Base definition inherited from FHIR specification
    #[serde(flatten)]
    pub base: BackboneElement,
    /// Role of participant in the appointment
    ///
    /// Binding: extensible (Role of participant in encounter.)
    ///
    /// ValueSet: http://hl7.org/fhir/ValueSet/encounter-participant-type
    #[serde(rename = "type")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Participation period of the actor
    pub period: Option<Period>,
    /// The individual, device, location, or service participating in the appointment
    pub actor: Option<Reference>,
    /// The participant is required to attend (optional when false)
    pub required: Option<BooleanType>,
    /// Extension element for the 'required' primitive field. Contains metadata and extensions.
    pub _required: Option<Element>,
    /// accepted | declined | tentative | needs-action
    pub status: Participationstatus,
    /// Extension element for the 'status' primitive field. Contains metadata and extensions.
    pub _status: Option<Element>,
}

impl Default for Appointment {
    fn default() -> Self {
        Self {
            base: DomainResource::default(),
            identifier: Default::default(),
            status: Appointmentstatus::default(),
            _status: Default::default(),
            cancellation_reason: Default::default(),
            class: Default::default(),
            service_category: Default::default(),
            service_type: Default::default(),
            specialty: Default::default(),
            appointment_type: Default::default(),
            reason: Default::default(),
            priority: Default::default(),
            description: Default::default(),
            _description: Default::default(),
            replaces: Default::default(),
            virtual_service: Default::default(),
            supporting_information: Default::default(),
            previous_appointment: Default::default(),
            originating_appointment: Default::default(),
            start: Default::default(),
            _start: Default::default(),
            end: Default::default(),
            _end: Default::default(),
            minutes_duration: Default::default(),
            _minutes_duration: Default::default(),
            requested_period: Default::default(),
            slot: Default::default(),
            account: Default::default(),
            created: Default::default(),
            _created: Default::default(),
            cancellation_date: Default::default(),
            _cancellation_date: Default::default(),
            note: Default::default(),
            patient_instruction: Default::default(),
            based_on: Default::default(),
            subject: Default::default(),
            participant: Vec::new(),
            recurrence_id: Default::default(),
            _recurrence_id: Default::default(),
            occurrence_changed: Default::default(),
            _occurrence_changed: Default::default(),
            recurrence_template: Default::default(),
        }
    }
}

impl Default for AppointmentRecurrencetemplateYearlytemplate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            year_interval: Default::default(),
            _year_interval: Default::default(),
        }
    }
}

impl Default for AppointmentRecurrencetemplate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            weekly_template: Default::default(),
            yearly_template: Default::default(),
            monthly_template: Default::default(),
            timezone: Default::default(),
            recurrence_type: Default::default(),
            last_occurrence_date: Default::default(),
            _last_occurrence_date: Default::default(),
            occurrence_count: Default::default(),
            _occurrence_count: Default::default(),
            occurrence_date: Default::default(),
            _occurrence_date: Default::default(),
            excluding_date: Default::default(),
            _excluding_date: Default::default(),
            excluding_recurrence_id: Default::default(),
            _excluding_recurrence_id: Default::default(),
        }
    }
}

impl Default for AppointmentRecurrencetemplateWeeklytemplate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            monday: Default::default(),
            _monday: Default::default(),
            tuesday: Default::default(),
            _tuesday: Default::default(),
            wednesday: Default::default(),
            _wednesday: Default::default(),
            thursday: Default::default(),
            _thursday: Default::default(),
            friday: Default::default(),
            _friday: Default::default(),
            saturday: Default::default(),
            _saturday: Default::default(),
            sunday: Default::default(),
            _sunday: Default::default(),
            week_interval: Default::default(),
            _week_interval: Default::default(),
        }
    }
}

impl Default for AppointmentRecurrencetemplateMonthlytemplate {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            day_of_month: Default::default(),
            _day_of_month: Default::default(),
            nth_week_of_month: Default::default(),
            day_of_week: Default::default(),
            month_interval: Default::default(),
            _month_interval: Default::default(),
        }
    }
}

impl Default for AppointmentParticipant {
    fn default() -> Self {
        Self {
            base: BackboneElement::default(),
            type_: Default::default(),
            period: Default::default(),
            actor: Default::default(),
            required: Default::default(),
            _required: Default::default(),
            status: Participationstatus::default(),
            _status: Default::default(),
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
    rh_foundation::Invariant::new("app-1", rh_foundation::Severity::Error, "Either the type or actor on the participant SHALL be specified", "type.exists() or actor.exists()"),
    rh_foundation::Invariant::new("app-2", rh_foundation::Severity::Error, "Either start and end are specified, or neither", "start.exists() = end.exists()"),
    rh_foundation::Invariant::new("app-3", rh_foundation::Severity::Error, "Only proposed or cancelled appointments can be missing start/end dates", "(start.exists() and end.exists()) or (status in ('proposed' | 'cancelled' | 'waitlist'))"),
    rh_foundation::Invariant::new("app-4", rh_foundation::Severity::Error, "Cancellation reason is only used for appointments that have been cancelled, or noshow", "cancellationReason.exists() implies (status='noshow' or status='cancelled')"),
    rh_foundation::Invariant::new("app-5", rh_foundation::Severity::Error, "The start must be less than or equal to the end", "start.exists() implies start <= end"),
    rh_foundation::Invariant::new("app-6", rh_foundation::Severity::Warning, "An appointment may have an originatingAppointment or recurrenceTemplate, but not both", "originatingAppointment.exists().not() or recurrenceTemplate.exists().not()"),
    rh_foundation::Invariant::new("app-7", rh_foundation::Severity::Error, "Cancellation date is only used for appointments that have been cancelled, or noshow", "cancellationDate.exists() implies (status='noshow' or status='cancelled')"),
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
                "Appointment.language",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/all-languages|5.0.0",
            )
            .with_description("IETF language tag for a human language"),
            rh_foundation::ElementBinding::new(
                "Appointment.participant.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/participationstatus|5.0.0",
            )
            .with_description("The Participation status of an appointment."),
            rh_foundation::ElementBinding::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.dayOfWeek",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/days-of-week|5.0.0",
            )
            .with_description("The days of the week."),
            rh_foundation::ElementBinding::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.nthWeekOfMonth",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/week-of-month|5.0.0",
            )
            .with_description("The set of weeks in a month."),
            rh_foundation::ElementBinding::new(
                "Appointment.recurrenceTemplate.timezone",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/timezones|5.0.0",
            )
            .with_description("IANA Timezones (BCP 175)"),
            rh_foundation::ElementBinding::new(
                "Appointment.status",
                rh_foundation::BindingStrength::Required,
                "http://hl7.org/fhir/ValueSet/appointmentstatus|5.0.0",
            )
            .with_description("The free/busy status of an appointment."),
        ]
    });

/// FHIR cardinality constraints for this resource/datatype
///
/// These define the minimum and maximum occurrences allowed for each element.
pub static CARDINALITIES: once_cell::sync::Lazy<Vec<rh_foundation::ElementCardinality>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            rh_foundation::ElementCardinality::new("Appointment.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.meta", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.implicitRules", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.language", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.text", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.contained", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.extension", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.modifierExtension", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.identifier", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.cancellationReason", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.class", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.serviceCategory", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.serviceType", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.specialty", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.appointmentType", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.reason", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.priority", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.description", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.replaces", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.virtualService", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.supportingInformation", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.previousAppointment", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Appointment.originatingAppointment",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new("Appointment.start", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.end", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.minutesDuration", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.requestedPeriod", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.slot", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.account", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.created", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.cancellationDate", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.note", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.patientInstruction", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.basedOn", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.subject", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant", 1, None),
            rh_foundation::ElementCardinality::new("Appointment.participant.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.extension", 0, None),
            rh_foundation::ElementCardinality::new(
                "Appointment.participant.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new("Appointment.participant.type", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.participant.period", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.actor", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.required", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.participant.status", 1, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.recurrenceId", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.occurrenceChanged", 0, Some(1)),
            rh_foundation::ElementCardinality::new("Appointment.recurrenceTemplate", 0, None),
            rh_foundation::ElementCardinality::new("Appointment.recurrenceTemplate.id", 0, Some(1)),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.timezone",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.recurrenceType",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.lastOccurrenceDate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.occurrenceCount",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.occurrenceDate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.monday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.tuesday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.wednesday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.thursday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.friday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.saturday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.sunday",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.weeklyTemplate.weekInterval",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.dayOfMonth",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.nthWeekOfMonth",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.dayOfWeek",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.monthlyTemplate.monthInterval",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.yearlyTemplate",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.yearlyTemplate.id",
                0,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.yearlyTemplate.extension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.yearlyTemplate.modifierExtension",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.yearlyTemplate.yearInterval",
                1,
                Some(1),
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.excludingDate",
                0,
                None,
            ),
            rh_foundation::ElementCardinality::new(
                "Appointment.recurrenceTemplate.excludingRecurrenceId",
                0,
                None,
            ),
        ]
    });

// Trait implementations
impl crate::traits::resource::ResourceAccessors for Appointment {
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

impl crate::traits::resource::ResourceMutators for Appointment {
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

impl crate::traits::resource::ResourceExistence for Appointment {
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

impl crate::traits::domain_resource::DomainResourceAccessors for Appointment {
    fn text(&self) -> Option<crate::datatypes::narrative::Narrative> {
        self.base.text.clone()
    }
    fn contained(&self) -> &[crate::resources::resource::Resource] {
        self.base.contained.as_deref().unwrap_or(&[])
    }
    fn extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.extension.as_deref().unwrap_or(&[])
    }
    fn modifier_extension(&self) -> &[crate::datatypes::extension::Extension] {
        self.base.modifier_extension.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::domain_resource::DomainResourceMutators for Appointment {
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
        resource.base.contained = Some(value);
        resource
    }
    fn add_contained(self, item: crate::resources::resource::Resource) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .contained
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.extension = Some(value);
        resource
    }
    fn add_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_modifier_extension(self, value: Vec<crate::datatypes::extension::Extension>) -> Self {
        let mut resource = self.clone();
        resource.base.modifier_extension = Some(value);
        resource
    }
    fn add_modifier_extension(self, item: crate::datatypes::extension::Extension) -> Self {
        let mut resource = self.clone();
        resource
            .base
            .modifier_extension
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::domain_resource::DomainResourceExistence for Appointment {
    fn has_text(&self) -> bool {
        self.base.text.is_some()
    }
    fn has_contained(&self) -> bool {
        self.base.contained.as_ref().is_some_and(|c| !c.is_empty())
    }
    fn has_extension(&self) -> bool {
        self.base.extension.as_ref().is_some_and(|e| !e.is_empty())
    }
    fn has_modifier_extension(&self) -> bool {
        self.base
            .modifier_extension
            .as_ref()
            .is_some_and(|m| !m.is_empty())
    }
}

impl crate::traits::appointment::AppointmentAccessors for Appointment {
    fn identifier(&self) -> &[Identifier] {
        self.identifier.as_deref().unwrap_or(&[])
    }
    fn status(&self) -> Appointmentstatus {
        self.status.clone()
    }
    fn cancellation_reason(&self) -> Option<CodeableConcept> {
        self.cancellation_reason.clone()
    }
    fn class(&self) -> &[CodeableConcept] {
        self.class.as_deref().unwrap_or(&[])
    }
    fn service_category(&self) -> &[CodeableConcept] {
        self.service_category.as_deref().unwrap_or(&[])
    }
    fn service_type(&self) -> &[CodeableReference] {
        self.service_type.as_deref().unwrap_or(&[])
    }
    fn specialty(&self) -> &[CodeableConcept] {
        self.specialty.as_deref().unwrap_or(&[])
    }
    fn appointment_type(&self) -> Option<CodeableConcept> {
        self.appointment_type.clone()
    }
    fn reason(&self) -> &[CodeableReference] {
        self.reason.as_deref().unwrap_or(&[])
    }
    fn priority(&self) -> Option<CodeableConcept> {
        self.priority.clone()
    }
    fn description(&self) -> Option<StringType> {
        self.description.clone()
    }
    fn replaces(&self) -> &[Reference] {
        self.replaces.as_deref().unwrap_or(&[])
    }
    fn virtual_service(&self) -> &[VirtualServiceDetail] {
        self.virtual_service.as_deref().unwrap_or(&[])
    }
    fn supporting_information(&self) -> &[Reference] {
        self.supporting_information.as_deref().unwrap_or(&[])
    }
    fn previous_appointment(&self) -> Option<Reference> {
        self.previous_appointment.clone()
    }
    fn originating_appointment(&self) -> Option<Reference> {
        self.originating_appointment.clone()
    }
    fn start(&self) -> Option<InstantType> {
        self.start.clone()
    }
    fn end(&self) -> Option<InstantType> {
        self.end.clone()
    }
    fn minutes_duration(&self) -> Option<PositiveIntType> {
        self.minutes_duration
    }
    fn requested_period(&self) -> &[Period] {
        self.requested_period.as_deref().unwrap_or(&[])
    }
    fn slot(&self) -> &[Reference] {
        self.slot.as_deref().unwrap_or(&[])
    }
    fn account(&self) -> &[Reference] {
        self.account.as_deref().unwrap_or(&[])
    }
    fn created(&self) -> Option<DateTimeType> {
        self.created.clone()
    }
    fn cancellation_date(&self) -> Option<DateTimeType> {
        self.cancellation_date.clone()
    }
    fn note(&self) -> &[Annotation] {
        self.note.as_deref().unwrap_or(&[])
    }
    fn patient_instruction(&self) -> &[CodeableReference] {
        self.patient_instruction.as_deref().unwrap_or(&[])
    }
    fn based_on(&self) -> &[Reference] {
        self.based_on.as_deref().unwrap_or(&[])
    }
    fn subject(&self) -> Option<Reference> {
        self.subject.clone()
    }
    fn participant(&self) -> &[AppointmentParticipant] {
        &self.participant
    }
    fn recurrence_id(&self) -> Option<PositiveIntType> {
        self.recurrence_id
    }
    fn occurrence_changed(&self) -> Option<BooleanType> {
        self.occurrence_changed
    }
    fn recurrence_template(&self) -> &[AppointmentRecurrencetemplate] {
        self.recurrence_template.as_deref().unwrap_or(&[])
    }
}

impl crate::traits::appointment::AppointmentMutators for Appointment {
    fn new() -> Self {
        Self::default()
    }
    fn set_identifier(self, value: Vec<Identifier>) -> Self {
        let mut resource = self.clone();
        resource.identifier = Some(value);
        resource
    }
    fn add_identifier(self, item: Identifier) -> Self {
        let mut resource = self.clone();
        resource.identifier.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_status(self, value: Appointmentstatus) -> Self {
        let mut resource = self.clone();
        resource.status = value;
        resource
    }
    fn set_cancellation_reason(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.cancellation_reason = Some(value);
        resource
    }
    fn set_class(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.class = Some(value);
        resource
    }
    fn add_class(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.class.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_service_category(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.service_category = Some(value);
        resource
    }
    fn add_service_category(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource
            .service_category
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_service_type(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.service_type = Some(value);
        resource
    }
    fn add_service_type(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource
            .service_type
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_specialty(self, value: Vec<CodeableConcept>) -> Self {
        let mut resource = self.clone();
        resource.specialty = Some(value);
        resource
    }
    fn add_specialty(self, item: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.specialty.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_appointment_type(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.appointment_type = Some(value);
        resource
    }
    fn set_reason(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.reason = Some(value);
        resource
    }
    fn add_reason(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource.reason.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_priority(self, value: CodeableConcept) -> Self {
        let mut resource = self.clone();
        resource.priority = Some(value);
        resource
    }
    fn set_description(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.description = Some(value);
        resource
    }
    fn set_replaces(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.replaces = Some(value);
        resource
    }
    fn add_replaces(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.replaces.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_virtual_service(self, value: Vec<VirtualServiceDetail>) -> Self {
        let mut resource = self.clone();
        resource.virtual_service = Some(value);
        resource
    }
    fn add_virtual_service(self, item: VirtualServiceDetail) -> Self {
        let mut resource = self.clone();
        resource
            .virtual_service
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_supporting_information(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.supporting_information = Some(value);
        resource
    }
    fn add_supporting_information(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource
            .supporting_information
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_previous_appointment(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.previous_appointment = Some(value);
        resource
    }
    fn set_originating_appointment(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.originating_appointment = Some(value);
        resource
    }
    fn set_start(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.start = Some(value);
        resource
    }
    fn set_end(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.end = Some(value);
        resource
    }
    fn set_minutes_duration(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.minutes_duration = Some(value);
        resource
    }
    fn set_requested_period(self, value: Vec<Period>) -> Self {
        let mut resource = self.clone();
        resource.requested_period = Some(value);
        resource
    }
    fn add_requested_period(self, item: Period) -> Self {
        let mut resource = self.clone();
        resource
            .requested_period
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_slot(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.slot = Some(value);
        resource
    }
    fn add_slot(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.slot.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_account(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.account = Some(value);
        resource
    }
    fn add_account(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.account.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_created(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.created = Some(value);
        resource
    }
    fn set_cancellation_date(self, value: String) -> Self {
        let mut resource = self.clone();
        resource.cancellation_date = Some(value);
        resource
    }
    fn set_note(self, value: Vec<Annotation>) -> Self {
        let mut resource = self.clone();
        resource.note = Some(value);
        resource
    }
    fn add_note(self, item: Annotation) -> Self {
        let mut resource = self.clone();
        resource.note.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_patient_instruction(self, value: Vec<CodeableReference>) -> Self {
        let mut resource = self.clone();
        resource.patient_instruction = Some(value);
        resource
    }
    fn add_patient_instruction(self, item: CodeableReference) -> Self {
        let mut resource = self.clone();
        resource
            .patient_instruction
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
    fn set_based_on(self, value: Vec<Reference>) -> Self {
        let mut resource = self.clone();
        resource.based_on = Some(value);
        resource
    }
    fn add_based_on(self, item: Reference) -> Self {
        let mut resource = self.clone();
        resource.based_on.get_or_insert_with(Vec::new).push(item);
        resource
    }
    fn set_subject(self, value: Reference) -> Self {
        let mut resource = self.clone();
        resource.subject = Some(value);
        resource
    }
    fn set_participant(self, value: Vec<AppointmentParticipant>) -> Self {
        let mut resource = self.clone();
        resource.participant = value;
        resource
    }
    fn add_participant(self, item: AppointmentParticipant) -> Self {
        let mut resource = self.clone();
        resource.participant.push(item);
        resource
    }
    fn set_recurrence_id(self, value: i32) -> Self {
        let mut resource = self.clone();
        resource.recurrence_id = Some(value);
        resource
    }
    fn set_occurrence_changed(self, value: bool) -> Self {
        let mut resource = self.clone();
        resource.occurrence_changed = Some(value);
        resource
    }
    fn set_recurrence_template(self, value: Vec<AppointmentRecurrencetemplate>) -> Self {
        let mut resource = self.clone();
        resource.recurrence_template = Some(value);
        resource
    }
    fn add_recurrence_template(self, item: AppointmentRecurrencetemplate) -> Self {
        let mut resource = self.clone();
        resource
            .recurrence_template
            .get_or_insert_with(Vec::new)
            .push(item);
        resource
    }
}

impl crate::traits::appointment::AppointmentExistence for Appointment {
    fn has_identifier(&self) -> bool {
        self.identifier.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_status(&self) -> bool {
        true
    }
    fn has_cancellation_reason(&self) -> bool {
        self.cancellation_reason.is_some()
    }
    fn has_class(&self) -> bool {
        self.class.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_service_category(&self) -> bool {
        self.service_category
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_service_type(&self) -> bool {
        self.service_type.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_specialty(&self) -> bool {
        self.specialty.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_appointment_type(&self) -> bool {
        self.appointment_type.is_some()
    }
    fn has_reason(&self) -> bool {
        self.reason.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_priority(&self) -> bool {
        self.priority.is_some()
    }
    fn has_description(&self) -> bool {
        self.description.is_some()
    }
    fn has_replaces(&self) -> bool {
        self.replaces.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_virtual_service(&self) -> bool {
        self.virtual_service.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_supporting_information(&self) -> bool {
        self.supporting_information
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_previous_appointment(&self) -> bool {
        self.previous_appointment.is_some()
    }
    fn has_originating_appointment(&self) -> bool {
        self.originating_appointment.is_some()
    }
    fn has_start(&self) -> bool {
        self.start.is_some()
    }
    fn has_end(&self) -> bool {
        self.end.is_some()
    }
    fn has_minutes_duration(&self) -> bool {
        self.minutes_duration.is_some()
    }
    fn has_requested_period(&self) -> bool {
        self.requested_period
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_slot(&self) -> bool {
        self.slot.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_account(&self) -> bool {
        self.account.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_created(&self) -> bool {
        self.created.is_some()
    }
    fn has_cancellation_date(&self) -> bool {
        self.cancellation_date.is_some()
    }
    fn has_note(&self) -> bool {
        self.note.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_patient_instruction(&self) -> bool {
        self.patient_instruction
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
    fn has_based_on(&self) -> bool {
        self.based_on.as_ref().is_some_and(|v| !v.is_empty())
    }
    fn has_subject(&self) -> bool {
        self.subject.is_some()
    }
    fn has_participant(&self) -> bool {
        !self.participant.is_empty()
    }
    fn has_recurrence_id(&self) -> bool {
        self.recurrence_id.is_some()
    }
    fn has_occurrence_changed(&self) -> bool {
        self.occurrence_changed.is_some()
    }
    fn has_recurrence_template(&self) -> bool {
        self.recurrence_template
            .as_ref()
            .is_some_and(|v| !v.is_empty())
    }
}

impl crate::validation::ValidatableResource for Appointment {
    fn resource_type(&self) -> &'static str {
        "Appointment"
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
        Some("http://hl7.org/fhir/StructureDefinition/Appointment")
    }
}

// Re-export traits for convenient importing
// This allows users to just import the resource module and get all associated traits
pub use crate::traits::appointment::{
    AppointmentAccessors, AppointmentExistence, AppointmentMutators,
};
