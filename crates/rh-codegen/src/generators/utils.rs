//! Utility functions for code generation
//!
//! This module contains utility functions used across different generators.

use crate::fhir_types::StructureDefinition;

/// Utility functions for code generation
pub struct GeneratorUtils;

impl GeneratorUtils {
    /// Convert a string to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                    None => String::new(),
                }
            })
            .collect()
    }

    /// Convert a PascalCase type name to snake_case for module imports
    pub fn to_snake_case(name: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = name.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            if ch.is_uppercase() && i > 0 {
                // Check if this is part of an acronym or start of a new word
                let is_acronym_continuation = i > 0 && chars[i - 1].is_uppercase();
                let is_followed_by_lowercase = i + 1 < chars.len() && chars[i + 1].is_lowercase();

                // Add underscore if:
                // 1. Previous char was lowercase (start of new word like "someWord")
                // 2. This is an acronym followed by lowercase (like "HTTPRequest" -> "http_request")
                if (i > 0 && chars[i - 1].is_lowercase())
                    || (is_acronym_continuation && is_followed_by_lowercase)
                {
                    result.push('_');
                }
            }

            result.push(ch.to_lowercase().next().unwrap());
        }

        result
    }

    /// Capitalize the first letter of a string
    pub fn capitalize_first_letter(s: &str) -> String {
        s[0..1].to_uppercase() + &s[1..]
    }

    /// Generate a proper Rust struct name from StructureDefinition URL or ID
    pub fn generate_struct_name(structure_def: &StructureDefinition) -> String {
        let raw_name = if structure_def.name == "alternate" {
            // Special case for "alternate" name - use ID
            Self::to_valid_rust_identifier(&structure_def.id)
        } else if structure_def.name.is_empty() {
            // No name provided - use ID
            Self::to_valid_rust_identifier(&structure_def.id)
        } else if structure_def.name != structure_def.id && !structure_def.id.is_empty() {
            // Name and ID differ - prefer ID for uniqueness, especially for extensions
            // This handles cases like cqf-library where name="library" but id="cqf-library"
            Self::to_valid_rust_identifier(&structure_def.id)
        } else {
            // Use name when it matches ID or ID is empty
            Self::to_valid_rust_identifier(&structure_def.name)
        };

        // FHIR conventions is to have capitalized names for non-primitive types
        if structure_def.kind != "primitive-type" {
            Self::capitalize_first_letter(&raw_name)
        } else {
            raw_name
        }
    }

    /// Convert a FHIR name to a valid Rust identifier while preserving the original as much as possible
    pub fn to_valid_rust_identifier(name: &str) -> String {
        // For names that are already valid Rust identifiers, use them as-is
        if Self::is_valid_rust_identifier(name) {
            return name.to_string();
        }

        // For names with spaces, dashes, or other characters, convert to PascalCase
        let mut result = String::new();
        let mut capitalize_next = true;

        for ch in name.chars() {
            if ch.is_alphanumeric() {
                if capitalize_next {
                    result.extend(ch.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(ch);
                }
            } else {
                // Skip non-alphanumeric characters and capitalize the next letter
                capitalize_next = true;
            }
        }

        // Ensure it starts with a letter or underscore (Rust requirement)
        if result.is_empty() || result.chars().next().unwrap().is_numeric() {
            result = format!("_{result}");
        }

        // Handle common FHIR acronyms that should remain uppercase
        Self::fix_acronyms(&result)
    }

    /// Fix common FHIR acronyms to maintain proper casing
    pub fn fix_acronyms(name: &str) -> String {
        let mut result = name.to_string();

        // Common FHIR acronyms that should be uppercase
        let acronyms = [
            ("Cqf", "CQF"),     // Clinical Quality Framework
            ("Fhir", "FHIR"),   // Fast Healthcare Interoperability Resources
            ("Hl7", "HL7"),     // Health Level 7
            ("Http", "HTTP"),   // HyperText Transfer Protocol
            ("Https", "HTTPS"), // HTTP Secure
            ("Json", "JSON"),   // JavaScript Object Notation
            ("Xml", "XML"),     // eXtensible Markup Language
            ("Uuid", "UUID"),   // Universally Unique Identifier
            ("Uri", "URI"),     // Uniform Resource Identifier
            ("Url", "URL"),     // Uniform Resource Locator
            ("Api", "API"),     // Application Programming Interface
        ];

        for (from, to) in &acronyms {
            result = result.replace(from, to);
        }

        result
    }

    /// Check if a string is a valid Rust identifier
    pub fn is_valid_rust_identifier(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        let mut chars = name.chars();
        let first_char = chars.next().unwrap();

        // First character must be a letter or underscore
        if !first_char.is_alphabetic() && first_char != '_' {
            return false;
        }

        // Remaining characters must be alphanumeric or underscore
        for ch in chars {
            if !ch.is_alphanumeric() && ch != '_' {
                return false;
            }
        }

        // Check if it's a Rust keyword
        !Self::is_rust_keyword(name)
    }

    /// Check if a string is a Rust keyword
    pub fn is_rust_keyword(name: &str) -> bool {
        matches!(
            name,
            "as" | "break"
                | "const"
                | "continue"
                | "crate"
                | "else"
                | "enum"
                | "extern"
                | "false"
                | "fn"
                | "for"
                | "if"
                | "impl"
                | "in"
                | "let"
                | "loop"
                | "match"
                | "mod"
                | "move"
                | "mut"
                | "pub"
                | "ref"
                | "return"
                | "self"
                | "Self"
                | "static"
                | "struct"
                | "super"
                | "trait"
                | "true"
                | "type"
                | "unsafe"
                | "use"
                | "where"
                | "while"
                | "async"
                | "await"
                | "dyn"
                | "abstract"
                | "become"
                | "box"
                | "do"
                | "final"
                | "macro"
                | "override"
                | "priv"
                | "typeof"
                | "unsized"
                | "virtual"
                | "yield"
                | "try"
        )
    }

    /// Convert a FHIR field name to a valid Rust field name
    pub fn to_rust_field_name(name: &str) -> String {
        // Handle FHIR choice types (fields ending with [x])
        let clean_name = if name.ends_with("[x]") {
            name.strip_suffix("[x]").unwrap_or(name)
        } else {
            name
        };

        // Convert to snake_case and handle Rust keywords
        let snake_case = clean_name
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    format!("_{}", c.to_lowercase())
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<String>();

        // Handle Rust keywords by appending underscore
        match snake_case.as_str() {
            "type" => "type_".to_string(),
            "use" => "use_".to_string(),
            "ref" => "ref_".to_string(),
            "mod" => "mod_".to_string(),
            "fn" => "fn_".to_string(),
            "let" => "let_".to_string(),
            "const" => "const_".to_string(),
            "static" => "static_".to_string(),
            "struct" => "struct_".to_string(),
            "enum" => "enum_".to_string(),
            "impl" => "impl_".to_string(),
            "trait" => "trait_".to_string(),
            "for" => "for_".to_string(),
            "if" => "if_".to_string(),
            "else" => "else_".to_string(),
            "while" => "while_".to_string(),
            "loop" => "loop_".to_string(),
            "match" => "match_".to_string(),
            "return" => "return_".to_string(),
            "where" => "where_".to_string(),
            "abstract" => "abstract_".to_string(),
            _ => snake_case,
        }
    }

    /// Convert a FHIR resource type name to filename using snake_case
    pub fn to_filename(structure_def: &StructureDefinition) -> String {
        // Use the struct name generation and convert to snake_case for filename
        let struct_name = Self::generate_struct_name(structure_def);
        let snake_case_name = Self::to_snake_case(&struct_name);

        format!("{snake_case_name}.rs")
    }

    /// Check if a type name represents a primitive Rust type
    pub fn is_primitive_type(type_name: &str) -> bool {
        matches!(
            type_name,
            "String" | "i32" | "u32" | "i64" | "u64" | "f32" | "f64" | "bool" | "usize" | "isize"
        )
    }

    /// Check if a type is a FHIR resource type
    pub fn is_fhir_resource_type(type_name: &str) -> bool {
        // Common FHIR resource types - using case-insensitive comparison
        matches!(
            type_name.to_lowercase().as_str(),
            "account"
                | "activitydefinition"
                | "actordefinition"
                | "administrableproductdefinition"
                | "adverseevent"
                | "allergyintolerance"
                | "appointment"
                | "appointmentresponse"
                | "artifactassessment"
                | "auditevent"
                | "basic"
                | "binary"
                | "biologicallyderivedproduct"
                | "biologicallyderivedproductdispense"
                | "bodystructure"
                | "bundle"
                | "canonicalresource"
                | "capabilitystatement"
                | "careplan"
                | "careteam"
                | "chargeitem"
                | "chargeitemdefinition"
                | "citation"
                | "claim"
                | "claimresponse"
                | "clinicalassessment"
                | "clinicalusedefinition"
                | "codesystem"
                | "communication"
                | "communicationrequest"
                | "compartmentdefinition"
                | "composition"
                | "conceptmap"
                | "condition"
                | "conditiondefinition"
                | "consent"
                | "contract"
                | "coverage"
                | "coverageeligibilityrequest"
                | "coverageeligibilityresponse"
                | "detectedissue"
                | "device"
                | "devicealert"
                | "deviceassociation"
                | "devicedefinition"
                | "devicedispense"
                | "devicemetric"
                | "devicerequest"
                | "deviceusage"
                | "diagnosticreport"
                | "documentreference"
                | "domainresource"
                | "encounter"
                | "encounterhistory"
                | "endpoint"
                | "enrollmentrequest"
                | "enrollmentresponse"
                | "episodeofcare"
                | "eventdefinition"
                | "evidence"
                | "evidencevariable"
                | "examplescenario"
                | "explanationofbenefit"
                | "familymemberhistory"
                | "flag"
                | "formularyitem"
                | "genomicstudy"
                | "goal"
                | "graphdefinition"
                | "group"
                | "guidanceresponse"
                | "healthcareservice"
                | "imagingselection"
                | "imagingstudy"
                | "immunization"
                | "immunizationevaluation"
                | "immunizationrecommendation"
                | "implementationguide"
                | "ingredient"
                | "insuranceplan"
                | "insuranceproduct"
                | "inventoryitem"
                | "inventoryreport"
                | "invoice"
                | "library"
                | "linkage"
                | "list"
                | "location"
                | "manufactureditemdefinition"
                | "measure"
                | "measurereport"
                | "medication"
                | "medicationadministration"
                | "medicationdispense"
                | "medicationknowledge"
                | "medicationrequest"
                | "medicationstatement"
                | "medicinalproductdefinition"
                | "messagedefinition"
                | "messageheader"
                | "metadataresource"
                | "moleculardefinition"
                | "molecularsequence"
                | "namingsystem"
                | "nutritionintake"
                | "nutritionorder"
                | "nutritionproduct"
                | "observation"
                | "observationdefinition"
                | "operationdefinition"
                | "operationoutcome"
                | "organization"
                | "organizationaffiliation"
                | "packagedproductdefinition"
                | "parameters"
                | "patient"
                | "paymentnotice"
                | "paymentreconciliation"
                | "permission"
                | "person"
                | "personalrelationship"
                | "plandefinition"
                | "practitioner"
                | "practitionerrole"
                | "procedure"
                | "provenance"
                | "questionnaire"
                | "questionnaireresponse"
                | "regulatedauthorization"
                | "relatedperson"
                | "requestorchestration"
                | "requirements"
                | "researchstudy"
                | "researchsubject"
                | "resource"
                | "riskassessment"
                | "schedule"
                | "searchparameter"
                | "servicerequest"
                | "slot"
                | "specimen"
                | "specimendefinition"
                | "structuredefinition"
                | "structuremap"
                | "subscription"
                | "subscriptionstatus"
                | "subscriptiontopic"
                | "substance"
                | "substancedefinition"
                | "substancenucleicacid"
                | "substancepolymer"
                | "substanceprotein"
                | "substancereferenceinformation"
                | "substancesourcematerial"
                | "supplydelivery"
                | "supplyrequest"
                | "task"
                | "terminologycapabilities"
                | "testplan"
                | "testreport"
                | "testscript"
                | "transport"
                | "valueset"
                | "verificationresult"
                | "visionprescription"
                | "bodysite"
                | "catalogentry"
                | "conformance"
                | "dataelement"
                | "devicecomponent"
                | "deviceuserequest"
                | "deviceusestatement"
                | "diagnosticorder"
                | "documentmanifest"
                | "effectevidencesynthesis"
                | "eligibilityrequest"
                | "eligibilityresponse"
                | "expansionprofile"
                | "imagingmanifest"
                | "imagingobjectselection"
                | "media"
                | "medicationorder"
                | "medicationusage"
                | "medicinalproduct"
                | "medicinalproductauthorization"
                | "medicinalproductcontraindication"
                | "medicinalproductindication"
                | "medicinalproductingredient"
                | "medicinalproductinteraction"
                | "medicinalproductmanufactured"
                | "medicinalproductpackaged"
                | "medicinalproductpharmaceutical"
                | "medicinalproductundesirableeffect"
                | "order"
                | "orderresponse"
                | "procedurerequest"
                | "processrequest"
                | "processresponse"
                | "referralrequest"
                | "requestgroup"
                | "researchdefinition"
                | "researchelementdefinition"
                | "riskevidencesynthesis"
                | "sequence"
                | "servicedefinition"
                | "substancespecification"
        )
    }

    /// Check if a type name represents a known FHIR data type
    pub fn is_fhir_datatype(name: &str) -> bool {
        matches!(
            name.to_lowercase().as_str(),
            "base"
                | "element"
                | "backboneelement"
                | "datatype"
                | "address"
                | "annotation"
                | "attachment"
                | "availability"
                | "backbonetype"
                | "dosage"
                | "elementdefinition"
                | "marketingstatus"
                | "productshelflife"
                | "relativetime"
                | "timing"
                | "codeableconcept"
                | "codeablereference"
                | "coding"
                | "contactdetail"
                | "contactpoint"
                | "contributor"
                | "datarequirement"
                | "extendedcontactdetail"
                | "humanname"
                | "identifier"
                | "monetarycomponent"
                | "parameterdefinition"
                | "period"
                | "quantity"
                | "range"
                | "ratio"
                | "ratiorange"
                | "reference"
                | "relatedartifact"
                | "sampleddata"
                | "signature"
                | "triggerdefinition"
                | "usagecontext"
                | "virtualservicedetail"
                | "base64binary"
                | "boolean"
                | "canonical"
                | "code"
                | "date"
                | "datetime"
                | "decimal"
                | "id"
                | "instant"
                | "integer"
                | "integer64"
                | "markdown"
                | "oid"
                | "positiveint"
                | "string"
                | "time"
                | "unsignedint"
                | "uri"
                | "url"
                | "uuid"
                | "xhtml"
                | "accountguarantor"
                | "accountdiagnosis"
                | "activitydefinitionparticipant"
                | "activitydefinitiondynamicvalue"
                | "actordefinitioninput"
                | "actordefinitionoutput"
                | "adverseeventparticipant"
                | "adverseeventsuspectentity"
                | "adverseeventcontributingfactor"
                | "adverseeventpreventiveaction"
                | "adverseeventmitigatingaction"
                | "adverseeventsupportinginfo"
                | "allergyintolerancereaction"
                | "appointmentparticipant"
                | "appointmentrecurrencetemplate"
                | "appointmentweeklytemplate"
                | "appointmentmonthlytemplate"
                | "appointmentyearlytemplate"
                | "appointmentrecurrencetemplateweeklytemplate"
                | "appointmentrecurrencetemplatemonthlytemplate"
                | "appointmentrecurrencetemplateyearlytemplate"
                | "artifactassessmentcontent"
                | "artifactassessmentrelatedartifact"
                | "auditeventagent"
                | "auditeventsource"
                | "auditevententity"
                | "basicextension"
                | "biologicallyderivedproductcollection"
                | "biologicallyderivedproductprocessing"
                | "biologicallyderivedproductmanipulation"
                | "biologicallyderivedproductstorage"
                | "biologicallyderivedproductdispenseperformer"
                | "bodystructureincludedstructure"
                | "bodystructureexcludedstructure"
                | "bundleentry"
                | "bundlelink"
                | "bundlerequest"
                | "bundleresponse"
                | "capabilitystatementsoftware"
                | "capabilitystatementimplementation"
                | "capabilitystatementrest"
                | "capabilitystatementmessaging"
                | "careplanactivity"
                | "careteamparticipant"
                | "chargeitemperformer"
                | "chargeitemdefinitionpropertygroup"
                | "citationsummary"
                | "citationabstract"
                | "citationcitedartifact"
                | "citationcontributorship"
                | "claimitem"
                | "claimsupportinginfo"
                | "claimdiagnosis"
                | "claimprocedure"
                | "claiminsurance"
                | "claimaccident"
                | "claimpayee"
                | "claimresponseitem"
                | "claimresponseinsurance"
                | "claimresponsepayment"
                | "claimresponseprocessnote"
                | "clinicalusedefinitionindication"
                | "clinicalusedefinitioncontraindication"
                | "clinicalusedefinitioninteraction"
                | "clinicalusedefinitionundesirableeffect"
                | "clinicalusedefinitionwarning"
                | "codesystemconcept"
                | "communicationpayload"
                | "communicationrequestpayload"
                | "compartmentdefinitionresource"
                | "compositionattester"
                | "compositionevent"
                | "compositionsection"
                | "conceptmapgroup"
                | "conceptmapadditionalattribute"
                | "conceptmaptarget"
                | "conditionstage"
                | "conditionevidence"
                | "consentpolicy"
                | "consentprovision"
                | "contractterm"
                | "contractfriendly"
                | "contractlegal"
                | "contractrule"
                | "coverageclass"
                | "coveragecosttobeneficiary"
                | "coverageeligibilityrequestitem"
                | "coverageeligibilityresponseitem"
                | "detectedissuemitigation"
                | "devicedevicename"
                | "deviceproperty"
                | "devicespecialization"
                | "deviceversion"
                | "devicedefinitionproperty"
                | "devicemetriccalibration"
                | "devicerequestparameter"
                | "deviceusageadherence"
                | "diagnosticreportmedia"
                | "documentreferencerelatesto"
                | "documentreferencecontent"
                | "documentreferencecontext"
                | "encounterparticipant"
                | "encounterreason"
                | "encounterdiagnosis"
                | "encounterlocation"
                | "endpointpayload"
                | "episodeofcarediagnosis"
                | "eventdefinitiontrigger"
                | "evidencevariablecharacteristic"
                | "examplescenarioactor"
                | "examplescenarioinstance"
                | "examplescenarioprocess"
                | "explanationofbenefititem"
                | "explanationofbenefitpayee"
                | "explanationofbenefitdiagnosis"
                | "explanationofbenefitprocedure"
                | "explanationofbenefitinsurance"
                | "explanationofbenefitaccident"
                | "explanationofbenefitprocessnote"
        )
    }

    /// Check if a type is a FHIR primitive type
    pub fn is_fhir_primitive_type(type_name: &str) -> bool {
        // FHIR primitive types that have extensions - matching actual generated type names
        matches!(
            type_name,
            "StringType"
                | "BooleanType"
                | "IntegerType"
                | "DecimalType"
                | "UriType"
                | "UrlType"
                | "CanonicalType"
                | "OidType"
                | "UuidType"
                | "InstantType"
                | "DateType"
                | "DateTimeType"
                | "TimeType"
                | "CodeType"
                | "IdType"
                | "MarkdownType"
                | "Base64BinaryType"
                | "UnsignedIntType"
                | "PositiveIntType"
                | "XhtmlType"
        )
    }

    /// Check if a type is a generated trait
    pub fn is_generated_trait(type_name: &str) -> bool {
        // Traits are typically generated for base types or common interfaces
        let lower_name = type_name.to_lowercase();
        lower_name.ends_with("trait")
            || matches!(
                lower_name.as_str(),
                "resourcetrait"
                    | "domainresourcetrait"
                    | "backboneelementtrait"
                    | "elementtrait"
                    | "metadataresourcetrait"
            )
    }

    /// Determine the correct import path for a given type name
    pub fn get_import_path_for_type(type_name: &str) -> String {
        // Check if it's a known FHIR resource type
        if Self::is_fhir_resource_type(type_name) {
            return format!(
                "crate::resource::{}::{}",
                Self::to_snake_case(type_name),
                type_name
            );
        }

        // Check if it's a known FHIR datatype (use existing method for consistency)
        if Self::is_fhir_datatype(type_name) {
            return format!(
                "crate::datatypes::{}::{}",
                Self::to_snake_case(type_name),
                type_name
            );
        }

        // Check if it's a known primitive type extension
        if Self::is_fhir_primitive_type(type_name) {
            // Map type names to correct module names
            let module_name = match type_name {
                "StringType" => "string",
                "BooleanType" => "boolean",
                "IntegerType" => "integer",
                "DecimalType" => "decimal",
                "UriType" => "uri",
                "UrlType" => "url",
                "CanonicalType" => "canonical",
                "OidType" => "oid",
                "UuidType" => "uuid",
                "InstantType" => "instant",
                "DateType" => "date",
                "DateTimeType" => "date_time",
                "TimeType" => "time",
                "CodeType" => "code",
                "IdType" => "id",
                "MarkdownType" => "markdown",
                "Base64BinaryType" => "base64binary",
                "UnsignedIntType" => "unsigned_int",
                "PositiveIntType" => "positive_int",
                "XhtmlType" => "xhtml",
                _ => "unknown_primitive",
            };
            return format!("crate::primitives::{module_name}::{type_name}");
        }

        // Check if it's a generated trait
        if Self::is_generated_trait(type_name) {
            return format!(
                "crate::traits::{}::{}",
                Self::to_snake_case(type_name),
                type_name
            );
        }

        // Default to bindings for unknown types (likely enums)
        format!(
            "crate::bindings::{}::{}",
            Self::to_snake_case(type_name),
            type_name
        )
    }
}
