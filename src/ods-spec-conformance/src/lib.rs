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

// ─────────────────────────────────────────────────────────────────────────────
// Spec integrity: link resolution and prose ↔ schema agreement.
//
// These guards exist because the specification drifted: prose and schema
// disagreed on code roles, memory tiers, enum members and traversal bounds,
// and every cross-chapter anchor in the glossary had rotted. Neither class of
// defect is detectable by JSON Schema validation alone.
// ─────────────────────────────────────────────────────────────────────────────

/// Every prose Markdown file in the specification surface (chapters, guides,
/// and the root documents), excluding build artifacts and test fixtures.
pub fn find_prose_files() -> Result<Vec<PathBuf>> {
    let root = find_workspace_root()?;
    let mut files = Vec::new();

    for entry in WalkDir::new(&root).into_iter().filter_entry(|e| {
        let name = e.file_name().to_string_lossy();
        !(name.starts_with('.') || name == "target" || name == "tests" || name == "node_modules")
    }) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|e| e == "md") {
            files.push(entry.into_path());
        }
    }

    files.sort();
    Ok(files)
}

/// Converts a Markdown heading into the anchor slug GitHub would generate:
/// lowercase, punctuation dropped, each remaining space turned into a hyphen.
///
/// Note that spaces are *not* collapsed — `## A & B` yields `a--b`, with two
/// hyphens, because the ampersand is removed but both spaces survive.
pub fn heading_slug(heading: &str) -> String {
    heading
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// Strips fenced code blocks and inline code spans, so that Markdown link
/// syntax shown as an *example* is not mistaken for a real link.
fn strip_code(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;

    // Fenced blocks.
    while let Some(start) = rest.find("```") {
        out.push_str(&rest[..start]);
        rest = match rest[start + 3..].find("```") {
            Some(end) => &rest[start + 3 + end + 3..],
            None => "",
        };
    }
    out.push_str(rest);

    // Inline spans.
    let mut result = String::with_capacity(out.len());
    let mut in_span = false;
    for ch in out.chars() {
        match ch {
            '`' => in_span = !in_span,
            '\n' => {
                in_span = false;
                result.push(ch);
            }
            _ if !in_span => result.push(ch),
            _ => {}
        }
    }
    result
}

/// A relative Markdown link that failed to resolve.
#[derive(Debug)]
pub struct BrokenLink {
    pub source: PathBuf,
    pub target: String,
    pub reason: &'static str,
}

impl std::fmt::Display for BrokenLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} -> {}", self.source.display(), self.reason, self.target)
    }
}

/// Resolves every relative Markdown link and `#anchor` across the prose surface.
pub fn find_broken_links() -> Result<Vec<BrokenLink>> {
    use std::collections::{HashMap, HashSet};

    let files = find_prose_files()?;

    let mut anchors: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    for path in &files {
        let content = std::fs::read_to_string(path)?;
        let set = content
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start_matches('#');
                if trimmed.len() < line.len() && trimmed.starts_with(' ') {
                    Some(heading_slug(trimmed))
                } else {
                    None
                }
            })
            .collect();
        anchors.insert(path.clone(), set);
    }

    let mut broken = Vec::new();
    for path in &files {
        let content = strip_code(&std::fs::read_to_string(path)?);
        let dir = path.parent().unwrap_or(path);

        for (target, anchor) in extract_relative_links(&content) {
            let resolved = normalize_path(&dir.join(&target));

            if !resolved.exists() {
                broken.push(BrokenLink {
                    source: path.clone(),
                    target: target.clone(),
                    reason: "missing file",
                });
                continue;
            }

            if let Some(anchor) = anchor {
                if resolved.extension().is_some_and(|e| e == "md") {
                    let known = anchors.get(&resolved);
                    if known.is_none_or(|set| !set.contains(&anchor)) {
                        broken.push(BrokenLink {
                            source: path.clone(),
                            target: format!("{}#{}", target, anchor),
                            reason: "missing anchor",
                        });
                    }
                }
            }
        }
    }

    Ok(broken)
}

/// Pulls `](target#anchor)` pairs out of Markdown, skipping absolute URLs and
/// same-page `#anchor` links.
fn extract_relative_links(content: &str) -> Vec<(String, Option<String>)> {
    let mut links = Vec::new();
    let bytes: Vec<char> = content.chars().collect();
    let mut i = 0;

    while i + 1 < bytes.len() {
        if bytes[i] == ']' && bytes[i + 1] == '(' {
            let start = i + 2;
            let mut end = start;
            let mut depth = 1;
            while end < bytes.len() {
                match bytes[end] {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    '\n' => break,
                    _ => {}
                }
                end += 1;
            }

            if end < bytes.len() && bytes[end] == ')' {
                let raw: String = bytes[start..end].iter().collect();
                let raw = raw.trim();
                let is_external = raw.starts_with("http://")
                    || raw.starts_with("https://")
                    || raw.starts_with('#')
                    || raw.starts_with("mailto:")
                    || raw.contains(char::is_whitespace);

                if !is_external && !raw.is_empty() {
                    match raw.split_once('#') {
                        Some((path, anchor)) => {
                            links.push((path.to_string(), Some(anchor.to_string())))
                        }
                        None => links.push((raw.to_string(), None)),
                    }
                }
            }
            i = end.max(i + 2);
        } else {
            i += 1;
        }
    }

    links
}

/// Resolves `.` and `..` segments without touching the filesystem.
fn normalize_path(path: &std::path::Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

/// Reads an enum out of a compiled schema by JSON pointer, as a sorted set.
pub fn schema_enum(schema_json: &Value, pointer: &str) -> Result<Vec<String>> {
    let node = schema_json
        .pointer(pointer)
        .ok_or_else(|| anyhow!("No schema node at pointer {}", pointer))?;

    let values = node
        .as_array()
        .ok_or_else(|| anyhow!("Schema node at {} is not an enum array", pointer))?;

    let mut out: Vec<String> = values
        .iter()
        .filter_map(|v| v.as_str().map(str::to_string))
        .collect();
    out.sort();
    Ok(out)
}

/// Collects every distinct `` `token` `` appearing in a named prose file, so a
/// documented vocabulary can be compared against the schema that enforces it.
pub fn prose_code_tokens(relative_path: &str) -> Result<Vec<String>> {
    let root = find_workspace_root()?;
    let content = std::fs::read_to_string(root.join(relative_path))
        .with_context(|| format!("Failed to read {}", relative_path))?;

    let mut tokens = Vec::new();
    let mut current: Option<String> = None;
    for ch in content.chars() {
        match (ch, &mut current) {
            ('`', None) => current = Some(String::new()),
            ('`', Some(buf)) => {
                tokens.push(std::mem::take(buf));
                current = None;
            }
            ('\n', Some(_)) => current = None,
            (c, Some(buf)) => buf.push(c),
            _ => {}
        }
    }

    tokens.sort();
    tokens.dedup();
    Ok(tokens)
}
