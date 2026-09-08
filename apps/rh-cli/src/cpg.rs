use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Args;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::output::{Envelope, OutputContext, OutputFormat};
use rh_cpg::apply::plan_definition::apply_plan_definition;
use rh_cpg::context::ApplyContext;
use rh_cpg::resolver::BundleResolver;

#[derive(Subcommand)]
pub enum CpgCommands {
    /// Apply a PlanDefinition via the FHIR $apply operation
    Apply(ApplyArgs),
}

#[derive(Args)]
pub struct ApplyArgs {
    /// Path to the PlanDefinition JSON file
    #[clap(long, value_name = "FILE")]
    plan_definition: PathBuf,

    /// Subject reference, e.g. "Patient/123"
    #[clap(long)]
    subject: String,

    /// Path to the content FHIR Bundle JSON (definitions, libraries, terminology)
    #[clap(long, value_name = "FILE")]
    content: PathBuf,

    /// Path to the data FHIR Bundle JSON (patient data)
    #[clap(long, value_name = "FILE")]
    data: Option<PathBuf>,

    /// Encounter reference, optional
    #[clap(long)]
    encounter: Option<String>,

    /// Practitioner reference, optional
    #[clap(long)]
    practitioner: Option<String>,

    /// Organization reference, optional
    #[clap(long)]
    organization: Option<String>,
}

pub async fn handle_command(cmd: CpgCommands, ctx: &OutputContext) -> Result<()> {
    let CpgCommands::Apply(args) = cmd;

    let plan_definition = read_json(&args.plan_definition)?;
    let content = read_json(&args.content)?;
    let content_bundle = as_content_bundle(content)?;
    let resolver = BundleResolver::new(&content_bundle)?;
    let mut context = ApplyContext::new(Arc::new(resolver), args.subject);

    if let Some(data) = args.data {
        context.data = Some(read_json(&data)?);
    }
    context.encounter = args.encounter;
    context.practitioner = args.practitioner;
    context.organization = args.organization;

    let result = apply_plan_definition(&plan_definition, &context)?;

    if ctx.is_json() {
        let envelope = Envelope::ok(result, "cpg apply");
        let json = if matches!(ctx.format, OutputFormat::Json) {
            serde_json::to_string_pretty(&envelope)?
        } else {
            serde_json::to_string(&envelope)?
        };
        println!("{json}");
    } else {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }

    Ok(())
}

fn read_json(path: &PathBuf) -> Result<Value> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))
}

fn as_content_bundle(value: Value) -> Result<Value> {
    if value.get("resourceType").and_then(Value::as_str) == Some("Bundle") {
        return Ok(value);
    }

    let resources = match value {
        Value::Array(resources) => resources,
        Value::Object(_) => vec![value],
        Value::Null => return Err(anyhow::anyhow!("content JSON must contain resources")),
        other => {
            return Err(anyhow::anyhow!(
                "content JSON must be a Bundle, resource object, or resource array; got {}",
                json_type_name(&other)
            ))
        }
    };

    let entries: Vec<Value> = resources
        .into_iter()
        .map(|resource| {
            let mut entry = Map::new();
            entry.insert("resource".to_string(), resource);
            Value::Object(entry)
        })
        .collect();

    Ok(json!({
        "resourceType": "Bundle",
        "type": "collection",
        "entry": entries,
    }))
}

fn json_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
