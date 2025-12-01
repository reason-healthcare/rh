use anyhow::Result;
use lru::LruCache;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::types::{IssueCode, ValidationIssue};
use crate::valueset::ValueSetLoader;

#[derive(Debug, Clone, Deserialize)]
pub struct Questionnaire {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub url: Option<String>,
    pub status: String,
    #[serde(default)]
    pub item: Vec<QuestionnaireItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuestionnaireItem {
    #[serde(rename = "linkId")]
    pub link_id: String,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub repeats: bool,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i32>,
    #[serde(default)]
    pub item: Vec<QuestionnaireItem>,
    #[serde(rename = "answerOption")]
    #[serde(default)]
    pub answer_option: Vec<AnswerOption>,
    #[serde(rename = "answerValueSet")]
    pub answer_value_set: Option<String>,
    #[serde(default)]
    pub extension: Vec<Extension>,
    #[serde(rename = "enableWhen")]
    #[serde(default)]
    pub enable_when: Vec<EnableWhenCondition>,
    #[serde(rename = "enableBehavior")]
    pub enable_behavior: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnswerOption {
    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,
    #[serde(rename = "valueString")]
    pub value_string: Option<String>,
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<i64>,
    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,
    #[serde(rename = "valueReference")]
    pub value_reference: Option<Value>,
    #[serde(default)]
    pub extension: Vec<Extension>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnableWhenCondition {
    pub question: String,
    pub operator: String,
    #[serde(rename = "answerBoolean")]
    pub answer_boolean: Option<bool>,
    #[serde(rename = "answerDecimal")]
    pub answer_decimal: Option<f64>,
    #[serde(rename = "answerInteger")]
    pub answer_integer: Option<i64>,
    #[serde(rename = "answerDate")]
    pub answer_date: Option<String>,
    #[serde(rename = "answerDateTime")]
    pub answer_date_time: Option<String>,
    #[serde(rename = "answerTime")]
    pub answer_time: Option<String>,
    #[serde(rename = "answerString")]
    pub answer_string: Option<String>,
    #[serde(rename = "answerCoding")]
    pub answer_coding: Option<Coding>,
    #[serde(rename = "answerQuantity")]
    pub answer_quantity: Option<Value>,
    #[serde(rename = "answerReference")]
    pub answer_reference: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Quantity {
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub system: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Extension {
    pub url: String,
    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<f64>,
    #[serde(rename = "valueInteger")]
    pub value_integer: Option<i64>,
    #[serde(rename = "valueString")]
    pub value_string: Option<String>,
    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,
    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<String>,
    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,
    #[serde(rename = "valueCode")]
    pub value_code: Option<String>,
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,
    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,
}

pub struct QuestionnaireLoader {
    cache: RwLock<LruCache<String, Questionnaire>>,
    package_dirs: Vec<PathBuf>,
}

impl QuestionnaireLoader {
    pub fn new(package_dirs: Vec<PathBuf>, cache_size: usize) -> Self {
        let capacity = NonZeroUsize::new(cache_size).unwrap_or(NonZeroUsize::new(50).unwrap());
        Self {
            cache: RwLock::new(LruCache::new(capacity)),
            package_dirs,
        }
    }

    pub fn load(&self, url: &str) -> Option<Questionnaire> {
        if let Some(cached) = self.cache.write().ok()?.get(url) {
            return Some(cached.clone());
        }

        for dir in &self.package_dirs {
            if let Some(q) = self.load_from_directory(url, dir) {
                self.cache.write().ok()?.put(url.to_string(), q.clone());
                return Some(q);
            }
        }

        None
    }

    pub fn load_from_json(&self, json: &Value) -> Option<Questionnaire> {
        serde_json::from_value(json.clone()).ok()
    }

    pub fn register(&self, questionnaire: Questionnaire) {
        if let Some(url) = &questionnaire.url {
            if let Ok(mut cache) = self.cache.write() {
                cache.put(url.clone(), questionnaire);
            }
        }
    }

    pub fn find_resource_type_for_url(&self, url: &str) -> Option<String> {
        for dir in &self.package_dirs {
            if let Some(resource_type) = self.find_resource_type_in_directory(url, dir) {
                return Some(resource_type);
            }
        }
        None
    }

    fn find_resource_type_in_directory(&self, url: &str, dir: &PathBuf) -> Option<String> {
        let entries = std::fs::read_dir(dir).ok()?;

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.extension().is_some_and(|ext| ext == "json") {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<Value>(&content) else {
                continue;
            };
            if value.get("url").and_then(|v| v.as_str()) == Some(url) {
                return value
                    .get("resourceType")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
            }
        }

        None
    }

    fn load_from_directory(&self, url: &str, dir: &PathBuf) -> Option<Questionnaire> {
        let entries = std::fs::read_dir(dir).ok()?;

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.extension().is_some_and(|ext| ext == "json") {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Ok(value) = serde_json::from_str::<Value>(&content) else {
                continue;
            };
            if value.get("resourceType").and_then(|v| v.as_str()) != Some("Questionnaire") {
                continue;
            }
            if value.get("url").and_then(|v| v.as_str()) == Some(url) {
                if let Ok(q) = serde_json::from_value(value) {
                    return Some(q);
                }
            }
        }

        None
    }
}

pub struct QuestionnaireResponseValidator<'a> {
    questionnaire: &'a Questionnaire,
    item_map: HashMap<String, &'a QuestionnaireItem>,
    valueset_loader: Option<&'a ValueSetLoader>,
}

impl<'a> QuestionnaireResponseValidator<'a> {
    pub fn new(questionnaire: &'a Questionnaire) -> Self {
        let mut item_map = HashMap::new();
        Self::build_item_map(&questionnaire.item, &mut item_map);
        Self {
            questionnaire,
            item_map,
            valueset_loader: None,
        }
    }

    pub fn with_valueset_loader(mut self, loader: &'a ValueSetLoader) -> Self {
        self.valueset_loader = Some(loader);
        self
    }

    fn build_item_map(
        items: &'a [QuestionnaireItem],
        map: &mut HashMap<String, &'a QuestionnaireItem>,
    ) {
        for item in items {
            map.insert(item.link_id.clone(), item);
            Self::build_item_map(&item.item, map);
        }
    }

    pub fn validate(&self, response: &Value) -> Result<Vec<ValidationIssue>> {
        let mut issues = Vec::new();

        let items = response
            .get("item")
            .and_then(|v| v.as_array())
            .map(|a| a.as_slice())
            .unwrap_or(&[]);

        self.validate_items(
            items,
            &self.questionnaire.item,
            "QuestionnaireResponse",
            &mut issues,
        )?;

        Self::check_required_items(
            &self.questionnaire.item,
            items,
            items, // all_response_items for enableWhen evaluation
            "QuestionnaireResponse",
            &mut issues,
        )?;

        Ok(issues)
    }

    fn validate_items(
        &self,
        response_items: &[Value],
        _questionnaire_items: &[QuestionnaireItem],
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        for (idx, item) in response_items.iter().enumerate() {
            let item_path = format!("{path}.item[{idx}]");

            let link_id = item.get("linkId").and_then(|v| v.as_str()).unwrap_or("");

            let q_item = match self.item_map.get(link_id) {
                Some(qi) => qi,
                None => {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Structure,
                            format!("LinkId '{link_id}' not found in questionnaire"),
                        )
                        .with_path(&item_path),
                    );
                    continue;
                }
            };

            let item_type = q_item.item_type.as_deref().unwrap_or("group");

            if item_type == "display" && item.get("answer").is_some() {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invariant,
                        "Items of type 'display' cannot have answers".to_string(),
                    )
                    .with_path(&item_path),
                );
            }

            if item_type == "group" && item.get("answer").is_some() {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invariant,
                        "Items of type 'group' cannot have direct answers".to_string(),
                    )
                    .with_path(&item_path),
                );
            }

            if item_type == "question" && item.get("answer").is_some() {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Structure,
                        "Items of type question should not have answers".to_string(),
                    )
                    .with_path(&item_path),
                );
            }

            let answers = item
                .get("answer")
                .and_then(|v| v.as_array())
                .map(|a| a.as_slice())
                .unwrap_or(&[]);

            self.validate_answer_cardinality(answers, q_item, &item_path, issues);
            self.validate_exclusive_options(answers, q_item, &item_path, issues);

            for (ans_idx, answer) in answers.iter().enumerate() {
                let ans_path = format!("{item_path}.answer[{ans_idx}]");
                self.validate_answer_type(answer, item_type, q_item, &ans_path, issues);
                self.validate_answer_option(answer, q_item, item_type, &ans_path, issues);
                self.validate_answer_value_set(answer, q_item, item_type, &ans_path, issues);
            }

            if let Some(sub_items) = item.get("item").and_then(|v| v.as_array()) {
                self.validate_items(sub_items, &q_item.item, &item_path, issues)?;
            }
        }

        Ok(())
    }

    fn validate_answer_type(
        &self,
        answer: &Value,
        expected_type: &str,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let actual_type = self.get_answer_type(answer);

        let type_mismatch = match expected_type {
            "boolean" => actual_type != Some("valueBoolean"),
            "decimal" => actual_type != Some("valueDecimal"),
            "integer" => actual_type != Some("valueInteger"),
            "date" => actual_type != Some("valueDate"),
            "dateTime" => actual_type != Some("valueDateTime"),
            "time" => actual_type != Some("valueTime"),
            "string" | "text" => actual_type != Some("valueString"),
            "url" => actual_type != Some("valueUri"),
            "choice" | "open-choice" => {
                actual_type != Some("valueCoding")
                    && actual_type != Some("valueString")
                    && actual_type != Some("valueInteger")
                    && actual_type != Some("valueDate")
                    && actual_type != Some("valueTime")
                    && actual_type != Some("valueReference")
            }
            "attachment" => actual_type != Some("valueAttachment"),
            "reference" => actual_type != Some("valueReference"),
            "quantity" => actual_type != Some("valueQuantity"),
            _ => false,
        };

        if type_mismatch {
            let actual_name = actual_type
                .map(|t| t.strip_prefix("value").unwrap_or(t))
                .unwrap_or("unknown");
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invariant,
                    format!("Answer value must be of the type {expected_type} not {actual_name}"),
                )
                .with_path(path),
            );
        }

        if expected_type == "decimal" {
            if let Some(val) = answer.get("valueDecimal") {
                self.validate_decimal_constraints(val, q_item, path, issues);
            }
        }

        if expected_type == "integer" {
            if let Some(val) = answer.get("valueInteger") {
                self.validate_integer_constraints(val, q_item, path, issues);
            }
        }

        if expected_type == "string" || expected_type == "text" {
            if let Some(val) = answer.get("valueString").and_then(|v| v.as_str()) {
                self.validate_string_constraints(val, q_item, path, issues);
            }
        }

        if expected_type == "date" || expected_type == "dateTime" {
            self.validate_date_constraints(answer, q_item, expected_type, path, issues);
        }

        if expected_type == "attachment" {
            if let Some(attachment) = answer.get("valueAttachment") {
                self.validate_attachment_constraints(attachment, q_item, path, issues);
            }
        }

        if expected_type == "quantity" {
            if let Some(quantity) = answer.get("valueQuantity") {
                self.validate_quantity_constraints(quantity, q_item, path, issues);
            }
        }

        if expected_type == "reference" {
            if let Some(reference) = answer.get("valueReference") {
                self.validate_reference_constraints(reference, q_item, path, issues);
            }
        }

        if expected_type == "url" {
            if let Some(uri) = answer.get("valueUri").and_then(|v| v.as_str()) {
                self.validate_uri_value(uri, path, issues);
            }
        }
    }

    fn get_answer_type(&self, answer: &Value) -> Option<&'static str> {
        if answer.get("valueBoolean").is_some() {
            Some("valueBoolean")
        } else if answer.get("valueDecimal").is_some() {
            Some("valueDecimal")
        } else if answer.get("valueInteger").is_some() {
            Some("valueInteger")
        } else if answer.get("valueDate").is_some() {
            Some("valueDate")
        } else if answer.get("valueDateTime").is_some() {
            Some("valueDateTime")
        } else if answer.get("valueTime").is_some() {
            Some("valueTime")
        } else if answer.get("valueString").is_some() {
            Some("valueString")
        } else if answer.get("valueUri").is_some() {
            Some("valueUri")
        } else if answer.get("valueCoding").is_some() {
            Some("valueCoding")
        } else if answer.get("valueAttachment").is_some() {
            Some("valueAttachment")
        } else if answer.get("valueReference").is_some() {
            Some("valueReference")
        } else if answer.get("valueQuantity").is_some() {
            Some("valueQuantity")
        } else {
            None
        }
    }

    fn validate_decimal_constraints(
        &self,
        value: &Value,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let val = match value.as_f64() {
            Some(v) => v,
            None => return,
        };

        for ext in &q_item.extension {
            if ext.url == "http://hl7.org/fhir/StructureDefinition/minValue" {
                if let Some(min) = ext.value_decimal {
                    if val < min {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Value,
                                format!(
                                    "The value {val} is less than the allowed minimum of {min}"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
            if ext.url == "http://hl7.org/fhir/StructureDefinition/maxValue" {
                if let Some(max) = ext.value_decimal {
                    if val > max {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Value,
                                format!(
                                    "The value {val} is greater than the allowed maximum of {max}"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
            if ext.url == "http://hl7.org/fhir/StructureDefinition/maxDecimalPlaces" {
                if let Some(max_places) = ext.value_integer {
                    let val_str = value.to_string();
                    if let Some(decimal_pos) = val_str.find('.') {
                        let decimal_places = val_str.len() - decimal_pos - 1;
                        if decimal_places > max_places as usize {
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::Value,
                                    format!(
                                        "The value {val_str} has too many decimal places (limit = {max_places})"
                                    ),
                                )
                                .with_path(path),
                            );
                        }
                    }
                }
            }
        }
    }

    fn validate_integer_constraints(
        &self,
        value: &Value,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let val = match value.as_i64() {
            Some(v) => v,
            None => return,
        };

        for ext in &q_item.extension {
            if ext.url == "http://hl7.org/fhir/StructureDefinition/minValue" {
                if let Some(min) = ext.value_integer {
                    if val < min {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Value,
                                format!(
                                    "The value {val} is less than the allowed minimum of {min}"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
            if ext.url == "http://hl7.org/fhir/StructureDefinition/maxValue" {
                if let Some(max) = ext.value_integer {
                    if val > max {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Value,
                                format!(
                                    "The value {val} is greater than the allowed maximum of {max}"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
        }
    }

    fn validate_attachment_constraints(
        &self,
        attachment: &Value,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let actual_size = attachment.get("size").and_then(|v| v.as_i64());
        let content_type = attachment
            .get("contentType")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        if let Some(data) = attachment.get("data").and_then(|v| v.as_str()) {
            let decoded_size = (data.len() * 3) / 4;
            if let Some(stated_size) = actual_size {
                if (stated_size as usize) != decoded_size
                    && ((stated_size as usize).abs_diff(decoded_size) > 2)
                {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Structure,
                            format!(
                                "Stated Attachment Size {stated_size} does not match actual attachment size {decoded_size}"
                            ),
                        )
                        .with_path(path),
                    );
                }
            }
        }

        for ext in &q_item.extension {
            if ext.url == "http://hl7.org/fhir/StructureDefinition/maxSize" {
                if let Some(max_size) = ext.value_decimal {
                    let size = actual_size.unwrap_or(0);
                    let allowed = max_size as i64;
                    if size > allowed {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Required,
                                format!("The attachment is too large (allowed = {allowed}, found = {size})"),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
            if ext.url == "http://hl7.org/fhir/StructureDefinition/mimeType" {
                if let Some(allowed_type) = &ext.value_code {
                    if !content_type.is_empty() && content_type != allowed_type {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Required,
                                format!("The mime type {content_type} is not valid for this answer (allowed = {allowed_type})"),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
        }
    }

    fn validate_quantity_constraints(
        &self,
        quantity: &Value,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        use rh_fhirpath::UnitConverter;

        let answer_value = quantity.get("value").and_then(|v| v.as_f64());
        let answer_unit = quantity.get("unit").and_then(|v| v.as_str());
        let answer_code = quantity.get("code").and_then(|v| v.as_str());

        let Some(answer_val) = answer_value else {
            return; // No value to validate
        };

        let unit_converter = UnitConverter::new();

        for ext in &q_item.extension {
            if ext.url
                == "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-minQuantity"
            {
                if let Some(ref min_qty) = ext.value_quantity {
                    self.validate_quantity_comparison(
                        answer_val,
                        answer_code,
                        answer_unit,
                        min_qty,
                        "minimum",
                        true, // is_min
                        &unit_converter,
                        path,
                        issues,
                    );
                }
            }
            if ext.url
                == "http://hl7.org/fhir/uv/sdc/StructureDefinition/sdc-questionnaire-maxQuantity"
            {
                if let Some(ref max_qty) = ext.value_quantity {
                    self.validate_quantity_comparison(
                        answer_val,
                        answer_code,
                        answer_unit,
                        max_qty,
                        "maximum",
                        false, // is_min
                        &unit_converter,
                        path,
                        issues,
                    );
                }
            }
        }

        // Validate unitOption constraints
        let unit_options: Vec<&Coding> = q_item
            .extension
            .iter()
            .filter(|ext| {
                ext.url == "http://hl7.org/fhir/StructureDefinition/questionnaire-unitOption"
            })
            .filter_map(|ext| ext.value_coding.as_ref())
            .collect();

        if !unit_options.is_empty() {
            let answer_unit_code = answer_code.or(answer_unit);

            if let Some(unit_code) = answer_unit_code {
                let is_valid_unit = unit_options
                    .iter()
                    .any(|opt| opt.code.as_deref() == Some(unit_code));

                if !is_valid_unit {
                    let allowed_units: Vec<String> = unit_options
                        .iter()
                        .filter_map(|opt| {
                            opt.code.as_ref().map(|c| {
                                if let Some(ref display) = opt.display {
                                    format!("{display} ({c})")
                                } else {
                                    c.clone()
                                }
                            })
                        })
                        .collect();

                    issues.push(
                        ValidationIssue::error(
                            IssueCode::CodeInvalid,
                            format!(
                                "Unit '{}' is not in the list of allowed units: {}",
                                unit_code,
                                allowed_units.join(", ")
                            ),
                        )
                        .with_path(path.to_string()),
                    );
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_quantity_comparison(
        &self,
        answer_value: f64,
        answer_code: Option<&str>,
        answer_unit: Option<&str>,
        constraint: &Quantity,
        constraint_name: &str,
        is_min: bool,
        unit_converter: &rh_fhirpath::UnitConverter,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let constraint_value = match constraint.value {
            Some(v) => v,
            None => return,
        };

        let constraint_code = constraint.code.as_deref();
        let answer_unit_display = answer_unit.unwrap_or("(no unit)");
        let constraint_unit_display = constraint.unit.as_deref().unwrap_or("(no unit)");

        // Only use actual UCUM codes for comparison, not just unit display strings
        // A formal UCUM code must be in the "code" field
        let has_ucum_codes = answer_code.is_some() && constraint_code.is_some();

        if !has_ucum_codes {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invariant,
                    format!(
                        "The quantity {answer_value} {answer_unit_display} cannot be compared to the allowed {constraint_name} of {constraint_value} {constraint_unit_display} because no formal units are specified"
                    ),
                )
                .with_path(path),
            );
            return;
        }

        let answer_code_str = answer_code.unwrap();
        let constraint_code_str = constraint_code.unwrap();

        // Check if units are compatible
        let answer_opt = Some(answer_code_str.to_string());
        let constraint_opt = Some(constraint_code_str.to_string());

        if !unit_converter.are_units_compatible(&answer_opt, &constraint_opt) {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invariant,
                    format!(
                        "The quantity {answer_value} {answer_unit_display} cannot be compared to the allowed {constraint_name} of {constraint_value} {constraint_unit_display} because the units are not comparable"
                    ),
                )
                .with_path(path),
            );
            return;
        }

        // Compare using unit conversion
        let comparison = unit_converter.compare_quantities(
            answer_value,
            &answer_opt,
            constraint_value,
            &constraint_opt,
        );

        match comparison {
            Ok(cmp) => {
                if is_min && cmp < 0 {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invariant,
                            format!(
                                "The quantity {answer_value} {answer_unit_display} (UCUM#{answer_code_str}) is less than the allowed minimum of {constraint_value} {constraint_unit_display} (UCUM#{constraint_code_str})"
                            ),
                        )
                        .with_path(path),
                    );
                } else if !is_min && cmp > 0 {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invariant,
                            format!(
                                "The quantity {answer_value} {answer_unit_display} (UCUM#{answer_code_str}) is greater than the allowed maximum of {constraint_value} {constraint_unit_display} (UCUM#{constraint_code_str})"
                            ),
                        )
                        .with_path(path),
                    );
                }
            }
            Err(_) => {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invariant,
                        format!(
                            "The quantity {answer_value} {answer_unit_display} cannot be compared to the allowed {constraint_name} of {constraint_value} {constraint_unit_display} because the units are not comparable"
                        ),
                    )
                    .with_path(path),
                );
            }
        }
    }

    fn validate_reference_constraints(
        &self,
        reference: &Value,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let reference_str = match reference.get("reference").and_then(|v| v.as_str()) {
            Some(r) => r,
            None => return,
        };

        if !self.is_valid_reference(reference_str) {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    format!("The reference '{reference_str}' is not a valid reference"),
                )
                .with_path(path),
            );
        }

        if let Some(resource_type) = self.extract_resource_type(reference_str) {
            if !self.is_valid_resource_type(&resource_type) {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invariant,
                        format!(
                            "The resource type {resource_type} in the reference {reference_str} is not valid"
                        ),
                    )
                    .with_path(path),
                );
            }

            let allowed_types: Vec<&str> = q_item
                .extension
                .iter()
                .filter(|ext| {
                    ext.url
                        == "http://hl7.org/fhir/StructureDefinition/questionnaire-referenceResource"
                })
                .filter_map(|ext| ext.value_code.as_deref())
                .collect();

            if !allowed_types.is_empty() && !allowed_types.contains(&resource_type.as_str()) {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invariant,
                        format!(
                            "The resource type '{}' in the reference {} is not allowed (allowed = {})",
                            resource_type,
                            reference_str,
                            allowed_types.join(",")
                        ),
                    )
                    .with_path(path),
                );
            }
        }
    }

    fn is_valid_reference(&self, reference: &str) -> bool {
        if reference.is_empty() {
            return false;
        }

        if reference.contains(' ') {
            return false;
        }

        if reference.starts_with("urn:uuid:") || reference.starts_with("urn:oid:") {
            return true;
        }

        if reference.starts_with('#') {
            return true;
        }

        if reference.starts_with("http://") || reference.starts_with("https://") {
            return true;
        }

        if reference.contains('/') {
            return true;
        }

        false
    }

    fn extract_resource_type(&self, reference: &str) -> Option<String> {
        if reference.starts_with("urn:") || reference.starts_with('#') {
            return None;
        }

        let path = if reference.starts_with("http://") || reference.starts_with("https://") {
            reference.split('/').rev().nth(1).map(|s| s.to_string())
        } else {
            reference.split('/').next().map(|s| s.to_string())
        };

        path.filter(|p| !p.is_empty() && p.chars().next().is_some_and(|c| c.is_uppercase()))
    }

    fn is_valid_resource_type(&self, resource_type: &str) -> bool {
        const VALID_RESOURCE_TYPES: &[&str] = &[
            "Account",
            "ActivityDefinition",
            "AdverseEvent",
            "AllergyIntolerance",
            "Appointment",
            "AppointmentResponse",
            "AuditEvent",
            "Basic",
            "Binary",
            "BiologicallyDerivedProduct",
            "BodyStructure",
            "Bundle",
            "CapabilityStatement",
            "CarePlan",
            "CareTeam",
            "CatalogEntry",
            "ChargeItem",
            "ChargeItemDefinition",
            "Claim",
            "ClaimResponse",
            "ClinicalImpression",
            "CodeSystem",
            "Communication",
            "CommunicationRequest",
            "CompartmentDefinition",
            "Composition",
            "ConceptMap",
            "Condition",
            "Consent",
            "Contract",
            "Coverage",
            "CoverageEligibilityRequest",
            "CoverageEligibilityResponse",
            "DetectedIssue",
            "Device",
            "DeviceDefinition",
            "DeviceMetric",
            "DeviceRequest",
            "DeviceUseStatement",
            "DiagnosticReport",
            "DocumentManifest",
            "DocumentReference",
            "EffectEvidenceSynthesis",
            "Encounter",
            "Endpoint",
            "EnrollmentRequest",
            "EnrollmentResponse",
            "EpisodeOfCare",
            "EventDefinition",
            "Evidence",
            "EvidenceVariable",
            "ExampleScenario",
            "ExplanationOfBenefit",
            "FamilyMemberHistory",
            "Flag",
            "Goal",
            "GraphDefinition",
            "Group",
            "GuidanceResponse",
            "HealthcareService",
            "ImagingStudy",
            "Immunization",
            "ImmunizationEvaluation",
            "ImmunizationRecommendation",
            "ImplementationGuide",
            "InsurancePlan",
            "Invoice",
            "Library",
            "Linkage",
            "List",
            "Location",
            "Measure",
            "MeasureReport",
            "Media",
            "Medication",
            "MedicationAdministration",
            "MedicationDispense",
            "MedicationKnowledge",
            "MedicationRequest",
            "MedicationStatement",
            "MedicinalProduct",
            "MedicinalProductAuthorization",
            "MedicinalProductContraindication",
            "MedicinalProductIndication",
            "MedicinalProductIngredient",
            "MedicinalProductInteraction",
            "MedicinalProductManufactured",
            "MedicinalProductPackaged",
            "MedicinalProductPharmaceutical",
            "MedicinalProductUndesirableEffect",
            "MessageDefinition",
            "MessageHeader",
            "MolecularSequence",
            "NamingSystem",
            "NutritionOrder",
            "Observation",
            "ObservationDefinition",
            "OperationDefinition",
            "OperationOutcome",
            "Organization",
            "OrganizationAffiliation",
            "Parameters",
            "Patient",
            "PaymentNotice",
            "PaymentReconciliation",
            "Person",
            "PlanDefinition",
            "Practitioner",
            "PractitionerRole",
            "Procedure",
            "Provenance",
            "Questionnaire",
            "QuestionnaireResponse",
            "RelatedPerson",
            "RequestGroup",
            "ResearchDefinition",
            "ResearchElementDefinition",
            "ResearchStudy",
            "ResearchSubject",
            "RiskAssessment",
            "RiskEvidenceSynthesis",
            "Schedule",
            "SearchParameter",
            "ServiceRequest",
            "Slot",
            "Specimen",
            "SpecimenDefinition",
            "StructureDefinition",
            "StructureMap",
            "Subscription",
            "Substance",
            "SubstanceNucleicAcid",
            "SubstancePolymer",
            "SubstanceProtein",
            "SubstanceReferenceInformation",
            "SubstanceSourceMaterial",
            "SubstanceSpecification",
            "SupplyDelivery",
            "SupplyRequest",
            "Task",
            "TerminologyCapabilities",
            "TestReport",
            "TestScript",
            "ValueSet",
            "VerificationResult",
            "VisionPrescription",
        ];

        VALID_RESOURCE_TYPES.contains(&resource_type)
    }

    fn validate_uri_value(&self, uri: &str, path: &str, issues: &mut Vec<ValidationIssue>) {
        if let Some(uuid_part) = uri.strip_prefix("urn:uuid:") {
            if !self.is_valid_uuid(uuid_part) {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invalid,
                        format!("UUIDs must be valid and lowercase ({uuid_part})"),
                    )
                    .with_path(path),
                );
            }
        }
    }

    fn is_valid_uuid(&self, uuid: &str) -> bool {
        if uuid.len() != 36 {
            return false;
        }

        let parts: Vec<&str> = uuid.split('-').collect();
        if parts.len() != 5 {
            return false;
        }

        if parts[0].len() != 8
            || parts[1].len() != 4
            || parts[2].len() != 4
            || parts[3].len() != 4
            || parts[4].len() != 12
        {
            return false;
        }

        for part in parts {
            if !part
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
            {
                return false;
            }
        }

        true
    }

    fn validate_string_constraints(
        &self,
        value: &str,
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        if let Some(max_len) = q_item.max_length {
            if value.len() > max_len as usize {
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Value,
                        format!(
                            "The string length {} exceeds the maximum length of {max_len}",
                            value.len()
                        ),
                    )
                    .with_path(path),
                );
            }
        }

        for ext in &q_item.extension {
            if ext.url == "http://hl7.org/fhir/StructureDefinition/regex" {
                if let Some(pattern) = &ext.value_string {
                    if let Ok(re) = regex::Regex::new(pattern) {
                        if !re.is_match(value) {
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::Value,
                                    format!(
                                        "The value '{value}' does not match the required pattern"
                                    ),
                                )
                                .with_path(path),
                            );
                        }
                    }
                }
            }
        }
    }

    fn validate_answer_cardinality(
        &self,
        answers: &[Value],
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let answer_count = answers.len();

        for ext in &q_item.extension {
            if ext.url == "http://hl7.org/fhir/StructureDefinition/questionnaire-minOccurs" {
                if let Some(min) = ext.value_integer {
                    if (answer_count as i64) < min {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Invalid,
                                format!(
                                    "The minimum number of answers is {min} but this has {answer_count} answers"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
            if ext.url == "http://hl7.org/fhir/StructureDefinition/questionnaire-maxOccurs" {
                if let Some(max) = ext.value_integer {
                    if (answer_count as i64) > max {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Invalid,
                                format!(
                                    "The maximum number of answers is {max} but this has {answer_count} answers"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
        }

        if !q_item.repeats && answer_count > 1 {
            issues.push(
                ValidationIssue::error(
                    IssueCode::Invalid,
                    format!("Only one answer is allowed but found {answer_count} answers"),
                )
                .with_path(path),
            );
        }
    }

    fn validate_exclusive_options(
        &self,
        answers: &[Value],
        q_item: &QuestionnaireItem,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        if answers.len() <= 1 || q_item.answer_option.is_empty() {
            return;
        }

        for (ans_idx, answer) in answers.iter().enumerate() {
            let is_exclusive = self.is_exclusive_option(answer, q_item);
            if is_exclusive {
                let answer_display = self.format_answer_display(answer);
                issues.push(
                    ValidationIssue::error(
                        IssueCode::Invariant,
                        format!(
                            "Selected answer {answer_display} is an exclusive option - can't select anything else at the same time"
                        ),
                    )
                    .with_path(format!("{path}.answer[{ans_idx}]")),
                );
                return;
            }
        }
    }

    fn is_exclusive_option(&self, answer: &Value, q_item: &QuestionnaireItem) -> bool {
        for opt in &q_item.answer_option {
            let is_exclusive = opt.extension.iter().any(|ext| {
                ext.url == "http://hl7.org/fhir/StructureDefinition/questionnaire-optionExclusive"
                    && ext.value_boolean == Some(true)
            });

            if !is_exclusive {
                continue;
            }

            if let Some(coding) = answer.get("valueCoding") {
                if let Some(opt_coding) = &opt.value_coding {
                    let sys_match = coding
                        .get("system")
                        .and_then(|v| v.as_str())
                        .zip(opt_coding.system.as_deref())
                        .map_or(true, |(a, b)| a == b);
                    let code_match = coding
                        .get("code")
                        .and_then(|v| v.as_str())
                        .zip(opt_coding.code.as_deref())
                        .map_or(true, |(a, b)| a == b);
                    if sys_match && code_match {
                        return true;
                    }
                }
            }
            if let Some(str_val) = answer.get("valueString").and_then(|v| v.as_str()) {
                if opt.value_string.as_deref() == Some(str_val) {
                    return true;
                }
            }
            if let Some(int_val) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
                if opt.value_integer == Some(int_val) {
                    return true;
                }
            }
        }
        false
    }

    fn format_answer_display(&self, answer: &Value) -> String {
        if let Some(coding) = answer.get("valueCoding") {
            let sys = coding.get("system").and_then(|v| v.as_str()).unwrap_or("");
            let code = coding.get("code").and_then(|v| v.as_str()).unwrap_or("");
            format!("{sys}#{code}")
        } else if let Some(s) = answer.get("valueString").and_then(|v| v.as_str()) {
            s.to_string()
        } else if let Some(i) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
            i.to_string()
        } else {
            "unknown".to_string()
        }
    }

    fn validate_answer_option(
        &self,
        answer: &Value,
        q_item: &QuestionnaireItem,
        item_type: &str,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        if q_item.answer_option.is_empty() {
            return;
        }

        if item_type == "open-choice" && answer.get("valueString").is_some() {
            return;
        }

        let is_valid = q_item.answer_option.iter().any(|opt| {
            if let Some(coding) = answer.get("valueCoding") {
                if let Some(opt_coding) = &opt.value_coding {
                    let sys_match = coding
                        .get("system")
                        .and_then(|v| v.as_str())
                        .zip(opt_coding.system.as_deref())
                        .map_or(true, |(a, b)| a == b);
                    let code_match = coding
                        .get("code")
                        .and_then(|v| v.as_str())
                        .zip(opt_coding.code.as_deref())
                        .map_or(true, |(a, b)| a == b);
                    return sys_match
                        && code_match
                        && coding.get("code").and_then(|v| v.as_str()).is_some()
                        && opt_coding.code.is_some();
                }
            }
            if let Some(str_val) = answer.get("valueString").and_then(|v| v.as_str()) {
                if let Some(opt_str) = &opt.value_string {
                    return str_val == opt_str;
                }
            }
            if let Some(int_val) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
                if let Some(opt_int) = opt.value_integer {
                    return int_val == opt_int;
                }
            }
            if let Some(date_val) = answer.get("valueDate").and_then(|v| v.as_str()) {
                if let Some(opt_date) = &opt.value_date {
                    return date_val == opt_date;
                }
            }
            false
        });

        if !is_valid {
            let answer_display = if let Some(coding) = answer.get("valueCoding") {
                let sys = coding.get("system").and_then(|v| v.as_str()).unwrap_or("");
                let code = coding.get("code").and_then(|v| v.as_str()).unwrap_or("");
                format!("{sys}::{code}")
            } else if let Some(s) = answer.get("valueString").and_then(|v| v.as_str()) {
                s.to_string()
            } else if let Some(i) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
                i.to_string()
            } else if let Some(d) = answer.get("valueDate").and_then(|v| v.as_str()) {
                d.to_string()
            } else {
                "unknown".to_string()
            };

            issues.push(
                ValidationIssue::error(
                    IssueCode::Invariant,
                    format!("The code {answer_display} is not in the set of permitted values"),
                )
                .with_path(path),
            );
        }
    }

    fn validate_answer_value_set(
        &self,
        answer: &Value,
        q_item: &QuestionnaireItem,
        item_type: &str,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let valueset_url = match &q_item.answer_value_set {
            Some(url) => url,
            None => return,
        };

        let loader = match self.valueset_loader {
            Some(l) => l,
            None => return,
        };

        if item_type == "open-choice" && answer.get("valueString").is_some() {
            return;
        }

        if let Some(coding) = answer.get("valueCoding") {
            let system = coding.get("system").and_then(|v| v.as_str()).unwrap_or("");
            let code = coding.get("code").and_then(|v| v.as_str()).unwrap_or("");

            if code.is_empty() {
                return;
            }

            match loader.contains_code(valueset_url, system, code) {
                Ok(true) => {}
                Ok(false) => {
                    let valueset_name = valueset_url.rsplit('/').next().unwrap_or(valueset_url);
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::CodeInvalid,
                            format!(
                                "The code '{code}' in the system '{system}' is not in the options value set ({valueset_name}) specified by the questionnaire"
                            ),
                        )
                        .with_path(path),
                    );
                }
                Err(_) => {}
            }
            return;
        }

        if let Some(value_string) = answer.get("valueString").and_then(|v| v.as_str()) {
            match loader.contains_string_value(valueset_url, value_string) {
                Ok(true) => {}
                Ok(false) => {
                    issues.push(
                        ValidationIssue::error(
                            IssueCode::Invariant,
                            format!(
                                "The value '{value_string}' is not in the list of allowed values (value set '{valueset_url}')"
                            ),
                        )
                        .with_path(format!("{path}.value.ofType(string)")),
                    );
                }
                Err(_) => {}
            }
        }
    }

    fn validate_date_constraints(
        &self,
        answer: &Value,
        q_item: &QuestionnaireItem,
        expected_type: &str,
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) {
        let field = if expected_type == "date" {
            "valueDate"
        } else {
            "valueDateTime"
        };
        let val = match answer.get(field).and_then(|v| v.as_str()) {
            Some(v) => v,
            None => return,
        };

        for ext in &q_item.extension {
            if ext.url == "http://hl7.org/fhir/StructureDefinition/minValue" {
                let min = ext
                    .value_date
                    .as_deref()
                    .or(ext.value_date_time.as_deref())
                    .or(ext.value_string.as_deref());
                if let Some(min) = min {
                    if val < min {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Value,
                                format!(
                                    "The value {val} is less than the allowed minimum of {min}"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
            if ext.url == "http://hl7.org/fhir/StructureDefinition/maxValue" {
                let max = ext
                    .value_date
                    .as_deref()
                    .or(ext.value_date_time.as_deref())
                    .or(ext.value_string.as_deref());
                if let Some(max) = max {
                    if val > max {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Value,
                                format!(
                                    "The value {val} is greater than the allowed maximum of {max}"
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }
        }
    }

    fn check_required_items(
        q_items: &[QuestionnaireItem],
        response_items: &[Value],
        all_response_items: &[Value],
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        for q_item in q_items {
            let found = response_items.iter().find(|ri| {
                ri.get("linkId")
                    .and_then(|v| v.as_str())
                    .is_some_and(|lid| lid == q_item.link_id)
            });

            // Check if enableWhen conditions are met before checking required
            if !q_item.enable_when.is_empty()
                && !Self::evaluate_enable_when(q_item, all_response_items)
            {
                // EnableWhen conditions not met - skip required validation for this item
                continue;
            }

            if q_item.required {
                if found.is_none() {
                    let item_type = q_item.item_type.as_deref().unwrap_or("group");
                    if item_type == "group" {
                        if !q_item.item.is_empty() {
                            issues.push(
                                ValidationIssue::error(
                                    IssueCode::Required,
                                    format!(
                                        "No response found for required group '{}'",
                                        q_item.link_id
                                    ),
                                )
                                .with_path(path),
                            );
                        }
                    } else {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Required,
                                format!(
                                    "No response answer found for required item '{}'",
                                    q_item.link_id
                                ),
                            )
                            .with_path(path),
                        );
                    }
                    continue;
                }

                let ri = found.unwrap();
                let item_type = q_item.item_type.as_deref().unwrap_or("group");

                if item_type == "group" && !q_item.item.is_empty() {
                    let has_sub_items = ri
                        .get("item")
                        .and_then(|v| v.as_array())
                        .is_some_and(|arr| !arr.is_empty());

                    if !has_sub_items {
                        let item_idx = response_items
                            .iter()
                            .position(|x| std::ptr::eq(x, ri))
                            .unwrap_or(0);
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Invariant,
                                "No sub-items found for required group".to_string(),
                            )
                            .with_path(format!("{path}.item[{item_idx}]")),
                        );
                    }
                }

                if item_type != "group" && item_type != "display" {
                    let has_answer = ri
                        .get("answer")
                        .and_then(|v| v.as_array())
                        .is_some_and(|arr| !arr.is_empty());

                    if !has_answer {
                        issues.push(
                            ValidationIssue::error(
                                IssueCode::Required,
                                format!(
                                    "No response answer found for required item '{}'",
                                    q_item.link_id
                                ),
                            )
                            .with_path(path),
                        );
                    }
                }
            }

            if let Some(ri) = found {
                if let Some(sub_items) = ri.get("item").and_then(|v| v.as_array()) {
                    let item_idx = response_items
                        .iter()
                        .position(|x| std::ptr::eq(x, ri))
                        .unwrap_or(0);
                    let item_path = format!("{path}.item[{item_idx}]");
                    Self::check_required_items(
                        &q_item.item,
                        sub_items,
                        all_response_items,
                        &item_path,
                        issues,
                    )?;
                }
            }
        }

        Ok(())
    }

    fn evaluate_enable_when(q_item: &QuestionnaireItem, all_response_items: &[Value]) -> bool {
        if q_item.enable_when.is_empty() {
            return true;
        }

        let behavior = q_item.enable_behavior.as_deref().unwrap_or("any");

        let condition_results: Vec<bool> = q_item
            .enable_when
            .iter()
            .map(|cond| Self::evaluate_single_condition(cond, all_response_items))
            .collect();

        match behavior {
            "all" => condition_results.iter().all(|&r| r),
            _ => condition_results.iter().any(|&r| r),
        }
    }

    fn evaluate_single_condition(cond: &EnableWhenCondition, all_response_items: &[Value]) -> bool {
        let answer = Self::find_answer_for_question(&cond.question, all_response_items);

        match cond.operator.as_str() {
            "exists" => {
                let expected_exists = cond.answer_boolean.unwrap_or(true);
                answer.is_some() == expected_exists
            }
            "=" => Self::compare_answer_equals(cond, answer),
            "!=" => !Self::compare_answer_equals(cond, answer),
            ">" => Self::compare_answer_greater(cond, answer),
            "<" => Self::compare_answer_less(cond, answer),
            ">=" => {
                Self::compare_answer_greater(cond, answer)
                    || Self::compare_answer_equals(cond, answer)
            }
            "<=" => {
                Self::compare_answer_less(cond, answer) || Self::compare_answer_equals(cond, answer)
            }
            _ => false,
        }
    }

    fn find_answer_for_question<'b>(
        question_link_id: &str,
        items: &'b [Value],
    ) -> Option<&'b Value> {
        for item in items {
            let link_id = item.get("linkId").and_then(|v| v.as_str()).unwrap_or("");
            if link_id == question_link_id {
                // Return the first answer if present
                if let Some(answers) = item.get("answer").and_then(|v| v.as_array()) {
                    return answers.first();
                }
                return None;
            }

            // Recursively search sub-items
            if let Some(sub_items) = item.get("item").and_then(|v| v.as_array()) {
                if let Some(found) = Self::find_answer_for_question(question_link_id, sub_items) {
                    return Some(found);
                }
            }
        }
        None
    }

    fn compare_answer_equals(cond: &EnableWhenCondition, answer: Option<&Value>) -> bool {
        let answer = match answer {
            Some(a) => a,
            None => return false,
        };

        if let Some(expected) = cond.answer_integer {
            if let Some(actual) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
                return actual == expected;
            }
        }

        if let Some(expected) = cond.answer_decimal {
            if let Some(actual) = answer.get("valueDecimal").and_then(|v| v.as_f64()) {
                return (actual - expected).abs() < f64::EPSILON;
            }
        }

        if let Some(expected) = cond.answer_boolean {
            if let Some(actual) = answer.get("valueBoolean").and_then(|v| v.as_bool()) {
                return actual == expected;
            }
        }

        if let Some(ref expected) = cond.answer_string {
            if let Some(actual) = answer.get("valueString").and_then(|v| v.as_str()) {
                return actual == expected;
            }
        }

        if let Some(ref expected) = cond.answer_date {
            if let Some(actual) = answer.get("valueDate").and_then(|v| v.as_str()) {
                return actual == expected;
            }
        }

        if let Some(ref expected) = cond.answer_date_time {
            if let Some(actual) = answer.get("valueDateTime").and_then(|v| v.as_str()) {
                return actual == expected;
            }
        }

        if let Some(ref expected) = cond.answer_time {
            if let Some(actual) = answer.get("valueTime").and_then(|v| v.as_str()) {
                return actual == expected;
            }
        }

        if let Some(ref expected_coding) = cond.answer_coding {
            if let Some(actual_coding) = answer.get("valueCoding") {
                let code_match = expected_coding.code.as_deref()
                    == actual_coding.get("code").and_then(|v| v.as_str());
                let system_match = expected_coding.system.is_none()
                    || expected_coding.system.as_deref()
                        == actual_coding.get("system").and_then(|v| v.as_str());
                return code_match && system_match;
            }
        }

        false
    }

    fn compare_answer_greater(cond: &EnableWhenCondition, answer: Option<&Value>) -> bool {
        let answer = match answer {
            Some(a) => a,
            None => return false,
        };

        if let Some(expected) = cond.answer_integer {
            if let Some(actual) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
                return actual > expected;
            }
        }

        if let Some(expected) = cond.answer_decimal {
            if let Some(actual) = answer.get("valueDecimal").and_then(|v| v.as_f64()) {
                return actual > expected;
            }
        }

        if let Some(ref expected) = cond.answer_date {
            if let Some(actual) = answer.get("valueDate").and_then(|v| v.as_str()) {
                return actual > expected.as_str();
            }
        }

        if let Some(ref expected) = cond.answer_date_time {
            if let Some(actual) = answer.get("valueDateTime").and_then(|v| v.as_str()) {
                return actual > expected.as_str();
            }
        }

        if let Some(ref expected) = cond.answer_time {
            if let Some(actual) = answer.get("valueTime").and_then(|v| v.as_str()) {
                return actual > expected.as_str();
            }
        }

        false
    }

    fn compare_answer_less(cond: &EnableWhenCondition, answer: Option<&Value>) -> bool {
        let answer = match answer {
            Some(a) => a,
            None => return false,
        };

        if let Some(expected) = cond.answer_integer {
            if let Some(actual) = answer.get("valueInteger").and_then(|v| v.as_i64()) {
                return actual < expected;
            }
        }

        if let Some(expected) = cond.answer_decimal {
            if let Some(actual) = answer.get("valueDecimal").and_then(|v| v.as_f64()) {
                return actual < expected;
            }
        }

        if let Some(ref expected) = cond.answer_date {
            if let Some(actual) = answer.get("valueDate").and_then(|v| v.as_str()) {
                return actual < expected.as_str();
            }
        }

        if let Some(ref expected) = cond.answer_date_time {
            if let Some(actual) = answer.get("valueDateTime").and_then(|v| v.as_str()) {
                return actual < expected.as_str();
            }
        }

        if let Some(ref expected) = cond.answer_time {
            if let Some(actual) = answer.get("valueTime").and_then(|v| v.as_str()) {
                return actual < expected.as_str();
            }
        }

        false
    }
}

impl Default for QuestionnaireLoader {
    fn default() -> Self {
        Self::new(vec![], 50)
    }
}
