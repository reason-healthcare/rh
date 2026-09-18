use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Args;
use clap::Subcommand;
use serde_json::Value;

use crate::output::{Envelope, OutputContext, OutputFormat};
use rh_cpg::apply::plan_definition::apply_plan_definition;
use rh_cpg::context::{ApplyContext, MeasurementPeriod};
use rh_cpg::resolver::{as_content_bundle, BundleResolver};

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

    /// RFC 3339 date-time or FHIR date used for deterministic CQL evaluation.
    #[clap(long)]
    evaluation_date: Option<String>,

    /// Inclusive Measurement Period start as an RFC 3339 date-time or FHIR date.
    #[clap(long, requires = "measurement_period_end")]
    measurement_period_start: Option<String>,

    /// Inclusive Measurement Period end as an RFC 3339 date-time or FHIR date.
    #[clap(long, requires = "measurement_period_start")]
    measurement_period_end: Option<String>,

    /// Additional CQL parameter as NAME=JSON. May be specified multiple times.
    #[clap(long, value_name = "NAME=JSON")]
    parameter: Vec<String>,
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
    context.evaluation_date = args.evaluation_date;
    context.measurement_period = args
        .measurement_period_start
        .zip(args.measurement_period_end)
        .map(|(start, end)| MeasurementPeriod {
            start,
            end,
            start_inclusive: true,
            end_inclusive: true,
        });
    context.parameters = parse_parameters(&args.parameter)?;

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

fn parse_parameters(values: &[String]) -> Result<HashMap<String, Value>> {
    values
        .iter()
        .map(|value| {
            let (name, json) = value
                .split_once('=')
                .with_context(|| format!("invalid --parameter '{value}': expected NAME=JSON"))?;
            if name.is_empty() {
                anyhow::bail!("invalid --parameter '{value}': parameter name is required");
            }
            let parsed = serde_json::from_str(json).with_context(|| {
                format!("invalid --parameter '{value}': value must be valid JSON")
            })?;
            Ok((name.to_string(), parsed))
        })
        .collect()
}

fn read_json(path: &PathBuf) -> Result<Value> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))
}
