use anyhow::{anyhow, Context, Result};
use jsonschema::Validator;
use serde_json::Value;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Finds the root directory of the ODS workspace (containing `ods.toml`).
pub fn find_workspace_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join("ods.toml").exists() {
            return Ok(current);
        }
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }
    Err(anyhow!("Could not locate workspace root with ods.toml"))
}

/// Extracts YAML frontmatter from a Markdown file and converts it into a `serde_json::Value`.
pub fn extract_frontmatter(markdown_content: &str) -> Result<Option<Value>> {
    let trimmed = markdown_content.trim_start();
    if !trimmed.starts_with("---") {
        return Ok(None);
    }

    let parts: Vec<&str> = markdown_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Ok(None);
    }

    let yaml_str = parts[1].trim();
    if yaml_str.is_empty() {
        return Ok(None);
    }

    let yaml_val: serde_yaml::Value = serde_yaml::from_str(yaml_str)
        .context("Failed to parse YAML frontmatter")?;
    
    let json_val: Value = serde_json::to_value(yaml_val)
        .context("Failed to convert YAML frontmatter to JSON")?;

    Ok(Some(json_val))
}

/// Loads a JSON schema from a workspace-relative path and compiles it.
pub fn load_compiled_schema(relative_path: &str) -> Result<(Value, Validator)> {
    let root = find_workspace_root()?;
    let schema_path = root.join(relative_path);
    let schema_str = std::fs::read_to_string(&schema_path)
        .with_context(|| format!("Failed to read schema at {:?}", schema_path))?;
    
    let schema_json: Value = serde_json::from_str(&schema_str)
        .with_context(|| format!("Failed to parse JSON in {:?}", schema_path))?;

    let compiled = jsonschema::validator_for(&schema_json)
        .map_err(|e| anyhow!("Failed to compile JSON schema {:?}: {}", schema_path, e))?;

    Ok((schema_json, compiled))
}

/// Validates a JSON value against a compiled JSON Schema.
pub fn validate_instance(schema: &Validator, instance: &Value) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for err in schema.iter_errors(instance) {
        errors.push(format!("path: {}, message: {}", err.instance_path, err));
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

/// Validates that `document.schema.json` contains `x-ods-supported-specs` and complete `x-ods-lifecycle` metadata.
pub fn verify_schema_lifecycle_metadata(schema_json: &Value) -> Result<(usize, usize)> {
    let supported_specs = schema_json
        .get("x-ods-supported-specs")
        .ok_or_else(|| anyhow!("Missing 'x-ods-supported-specs' at root of document.schema.json"))?;

    if supported_specs.get("ods").is_none() || supported_specs.get("okf").is_none() {
        return Err(anyhow!("x-ods-supported-specs must declare both 'ods' and 'okf' specifications"));
    }

    let mut total_props = 0;
    let mut missing_lifecycle = Vec::new();

    if let Some(props) = schema_json.get("properties").and_then(|p| p.as_object()) {
        for (name, def) in props {
            if name == "ods" {
                if let Some(ods_props) = def.get("properties").and_then(|p| p.as_object()) {
                    for (ods_name, ods_def) in ods_props {
                        total_props += 1;
                        if ods_def.get("x-ods-lifecycle").is_none() {
                            missing_lifecycle.push(format!("ods.{}", ods_name));
                        }
                    }
                }
            } else {
                total_props += 1;
                if def.get("x-ods-lifecycle").is_none() {
                    missing_lifecycle.push(name.clone());
                }
            }
        }
    }

    if !missing_lifecycle.is_empty() {
        return Err(anyhow!(
            "Properties missing 'x-ods-lifecycle' metadata: {:?}",
            missing_lifecycle
        ));
    }

    Ok((total_props, 2))
}

/// Scans a directory for all markdown files.
pub fn find_markdown_files(relative_dir: &str) -> Result<Vec<PathBuf>> {
    let root = find_workspace_root()?;
    let target_dir = root.join(relative_dir);
    let mut files = Vec::new();

    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    files.push(entry.into_path());
                }
            }
        }
    }

    files.sort();
    Ok(files)
}
