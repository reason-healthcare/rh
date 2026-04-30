//! Shell hook processor — runs an external command as a pipeline stage.
//!
//! Resources are exchanged with the external process via a temporary directory
//! (`$PACKAGER_WORKDIR/resources/`). See `PROCESSORS.md` for the full contract.

use crate::{
    config::ShellProcessorConfig, context::PublishContext, hooks::HookProcessor, PublisherError,
    Result,
};
use tracing::{debug, info, warn};

/// Hook processor that runs an arbitrary shell command.
///
/// Configured under `[processors.<name>]` in `packager.toml`.
pub struct ShellProcessor {
    processor_name: String,
    config: ShellProcessorConfig,
}

impl ShellProcessor {
    pub fn new(name: String, config: ShellProcessorConfig) -> Self {
        Self {
            processor_name: name,
            config,
        }
    }
}

impl HookProcessor for ShellProcessor {
    fn name(&self) -> &str {
        &self.processor_name
    }

    fn run(&self, ctx: &mut PublishContext) -> Result<()> {
        // Create a temporary directory owned by this processor invocation.
        let tmp = tempfile::TempDir::new()?;
        let workdir = tmp.path();
        let resources_dir = workdir.join("resources");
        std::fs::create_dir_all(&resources_dir)?;

        // Serialize all in-memory resources into the workdir so the script can read/modify them.
        for (stem, resource) in &ctx.resources {
            let json = serde_json::to_string_pretty(resource)?;
            std::fs::write(resources_dir.join(format!("{stem}.json")), json)?;
        }

        // Determine the command's working directory (relative to source_dir).
        let cwd = match &self.config.working_dir {
            Some(wd) => ctx.source_dir.join(wd),
            None => ctx.source_dir.clone(),
        };

        info!(
            processor = %self.processor_name,
            command = %self.config.command,
            cwd = %cwd.display(),
            "Running shell processor"
        );

        // Run the command via the OS shell so that shell features (piping, env
        // expansion, etc.) work naturally.
        let mut cmd = build_command(&self.config.command);
        cmd.current_dir(&cwd);

        // Inject packager context variables.
        cmd.env("PACKAGER_SOURCE_DIR", ctx.source_dir.as_os_str());
        cmd.env("PACKAGER_OUTPUT_DIR", ctx.output_dir.as_os_str());
        cmd.env("PACKAGER_WORKDIR", workdir.as_os_str());
        cmd.env("PACKAGER_PACKAGE_NAME", &ctx.package_json.name);
        cmd.env("PACKAGER_PACKAGE_VERSION", &ctx.package_json.version);
        cmd.env(
            "PACKAGER_FHIR_VERSIONS",
            ctx.package_json.fhir_versions.join(","),
        );

        // User-supplied overrides applied last (highest precedence).
        for (k, v) in &self.config.env {
            cmd.env(k, v);
        }

        let output = cmd.output().map_err(|e| {
            PublisherError::Other(anyhow::anyhow!(
                "Failed to spawn shell processor '{}': {}",
                self.processor_name,
                e
            ))
        })?;

        // Forward stdout as info-level log lines.
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            info!(processor = %self.processor_name, "{line}");
        }

        // Forward stderr as warn-level log lines.
        let stderr = String::from_utf8_lossy(&output.stderr);
        for line in stderr.lines() {
            warn!(processor = %self.processor_name, "{line}");
        }

        if !output.status.success() {
            let code = output
                .status
                .code()
                .map_or_else(|| "signal".to_string(), |c| c.to_string());
            let snippet = stderr.lines().next().unwrap_or("(no stderr)");
            return Err(PublisherError::Other(anyhow::anyhow!(
                "Shell processor '{}' exited with code {code}: {snippet}",
                self.processor_name,
            )));
        }

        // Read back every JSON file from resources_dir and upsert into ctx.resources.
        // Deletions are not possible — the script cannot remove resources from the pipeline.
        let mut updated = 0usize;
        for entry in std::fs::read_dir(&resources_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            let content = std::fs::read_to_string(&path)?;
            match serde_json::from_str(&content) {
                Ok(value) => {
                    ctx.resources.insert(stem.to_string(), value);
                    updated += 1;
                }
                Err(e) => {
                    warn!(
                        processor = %self.processor_name,
                        "Skipping malformed JSON for resource '{stem}' written by shell processor: {e}"
                    );
                }
            }
        }

        debug!(
            processor = %self.processor_name,
            resources_synced = updated,
            "Shell processor complete"
        );

        // tmp is dropped here, cleaning up the temp directory automatically.
        Ok(())
    }
}

/// Build a `Command` that runs `cmd_str` through the OS shell.
fn build_command(cmd_str: &str) -> std::process::Command {
    if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.args(["/C", cmd_str]);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.args(["-c", cmd_str]);
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::PublisherConfig, manifest::PackageJson, processors::shell::ShellProcessor,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn make_ctx(tmp: &TempDir) -> PublishContext {
        let mut resources = HashMap::new();
        resources.insert(
            "StructureDefinition-foo".to_string(),
            json!({"resourceType": "StructureDefinition", "id": "foo", "status": "draft"}),
        );
        PublishContext {
            source_dir: tmp.path().to_path_buf(),
            input_dir: tmp.path().to_path_buf(),
            output_dir: tmp.path().join("output"),
            package_json: PackageJson {
                name: "test.pkg".to_string(),
                version: "1.0.0".to_string(),
                fhir_versions: vec!["4.0.1".to_string()],
                dependencies: HashMap::new(),
                url: None,
                description: None,
                author: None,
                license: None,
                extra: HashMap::new(),
            },
            resources,
            examples: HashMap::new(),
            config: PublisherConfig::default(),
            standalone_markdown: Vec::new(),
        }
    }

    #[test]
    fn shell_processor_name_matches_constructor() {
        let p = ShellProcessor::new("my-script".to_string(), Default::default());
        assert_eq!(p.name(), "my-script");
    }

    #[test]
    fn exits_zero_succeeds() {
        let tmp = TempDir::new().unwrap();
        let mut ctx = make_ctx(&tmp);
        let p = ShellProcessor::new(
            "echo-test".to_string(),
            crate::config::ShellProcessorConfig {
                command: "exit 0".to_string(),
                ..Default::default()
            },
        );
        p.run(&mut ctx).unwrap();
    }

    #[test]
    fn exits_nonzero_returns_error() {
        let tmp = TempDir::new().unwrap();
        let mut ctx = make_ctx(&tmp);
        let p = ShellProcessor::new(
            "fail-test".to_string(),
            crate::config::ShellProcessorConfig {
                command: "exit 1".to_string(),
                ..Default::default()
            },
        );
        let err = p.run(&mut ctx).unwrap_err();
        assert!(err.to_string().contains("exit"));
    }

    #[test]
    fn script_can_add_resource() {
        let tmp = TempDir::new().unwrap();
        let mut ctx = make_ctx(&tmp);

        // Script writes a new resource JSON into $PACKAGER_WORKDIR/resources/
        let cmd = r#"echo '{"resourceType":"ValueSet","id":"new-vs"}' > "$PACKAGER_WORKDIR/resources/ValueSet-new.json""#;
        let p = ShellProcessor::new(
            "add-resource".to_string(),
            crate::config::ShellProcessorConfig {
                command: cmd.to_string(),
                ..Default::default()
            },
        );
        p.run(&mut ctx).unwrap();
        assert!(ctx.resources.contains_key("ValueSet-new"));
    }

    #[test]
    fn script_can_modify_resource() {
        let tmp = TempDir::new().unwrap();
        let mut ctx = make_ctx(&tmp);

        // Script overwrites StructureDefinition-foo.json with status=active
        let cmd = r#"python3 -c "
import json, os
p = os.path.join(os.environ['PACKAGER_WORKDIR'], 'resources', 'StructureDefinition-foo.json')
r = json.load(open(p))
r['status'] = 'active'
json.dump(r, open(p, 'w'))
""#;
        let p = ShellProcessor::new(
            "modify-resource".to_string(),
            crate::config::ShellProcessorConfig {
                command: cmd.to_string(),
                ..Default::default()
            },
        );
        p.run(&mut ctx).unwrap();
        let status = ctx.resources["StructureDefinition-foo"]["status"]
            .as_str()
            .unwrap();
        assert_eq!(status, "active");
    }

    #[test]
    fn env_vars_are_passed_to_script() {
        let tmp = TempDir::new().unwrap();
        let mut ctx = make_ctx(&tmp);

        let out_file = tmp.path().join("env_check.txt");
        let cmd = format!(r#"echo "$MY_CUSTOM_VAR" > "{}""#, out_file.display());
        let mut env = HashMap::new();
        env.insert(
            "MY_CUSTOM_VAR".to_string(),
            "hello-from-packager".to_string(),
        );

        let p = ShellProcessor::new(
            "env-test".to_string(),
            crate::config::ShellProcessorConfig {
                command: cmd,
                env,
                ..Default::default()
            },
        );
        p.run(&mut ctx).unwrap();
        let contents = std::fs::read_to_string(&out_file).unwrap();
        assert!(contents.contains("hello-from-packager"));
    }
}
