use anyhow::Result;
use lru::LruCache;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::types::{IssueCode, ValidationIssue};

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
    #[serde(default)]
    pub extension: Vec<Extension>,
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
}

#[derive(Debug, Clone, Deserialize)]
pub struct Coding {
    pub system: Option<String>,
    pub code: Option<String>,
    pub display: Option<String>,
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
}

impl<'a> QuestionnaireResponseValidator<'a> {
    pub fn new(questionnaire: &'a Questionnaire) -> Self {
        let mut item_map = HashMap::new();
        Self::build_item_map(&questionnaire.item, &mut item_map);
        Self {
            questionnaire,
            item_map,
        }
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

            let answers = item
                .get("answer")
                .and_then(|v| v.as_array())
                .map(|a| a.as_slice())
                .unwrap_or(&[]);

            self.validate_answer_cardinality(answers, q_item, &item_path, issues);

            for (ans_idx, answer) in answers.iter().enumerate() {
                let ans_path = format!("{item_path}.answer[{ans_idx}]");
                self.validate_answer_type(answer, item_type, q_item, &ans_path, issues);
                self.validate_answer_option(answer, q_item, item_type, &ans_path, issues);
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
        path: &str,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<()> {
        for q_item in q_items {
            let found = response_items.iter().find(|ri| {
                ri.get("linkId")
                    .and_then(|v| v.as_str())
                    .is_some_and(|lid| lid == q_item.link_id)
            });

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
                    Self::check_required_items(&q_item.item, sub_items, &item_path, issues)?;
                }
            }
        }

        Ok(())
    }
}

impl Default for QuestionnaireLoader {
    fn default() -> Self {
        Self::new(vec![], 50)
    }
}
