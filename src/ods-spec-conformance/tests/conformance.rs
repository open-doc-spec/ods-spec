use anyhow::Result;
use serde_json::Value;
use ods_spec_conformance::*;

const DOC_SCHEMA: &str = "schemas/2.0.0/document.schema.json";
const CONFIG_SCHEMA: &str = "schemas/2.0.0/config.schema.json";
const PROFILE_SCHEMA: &str = "schemas/2.0.0/profile.schema.json";
const DOC_SCHEMA_21: &str = "schemas/2.1.0/document.schema.json";
const CONFIG_SCHEMA_21: &str = "schemas/2.1.0/config.schema.json";
const PROFILE_SCHEMA_21: &str = "schemas/2.1.0/profile.schema.json";

#[test]
fn test_all_schemas_compile_cleanly() -> Result<()> {
    let _ = load_compiled_schema(DOC_SCHEMA)?;
    let _ = load_compiled_schema(CONFIG_SCHEMA)?;
    let _ = load_compiled_schema(PROFILE_SCHEMA)?;
    let _ = load_compiled_schema(DOC_SCHEMA_21)?;
    let _ = load_compiled_schema(CONFIG_SCHEMA_21)?;
    let _ = load_compiled_schema(PROFILE_SCHEMA_21)?;
    Ok(())
}

#[test]
fn test_lifecycle_metadata_coverage() -> Result<()> {
    let (doc_schema_json, _) = load_compiled_schema(DOC_SCHEMA)?;
    let spec_count = doc_schema_json
        .get("x-ods-supported-specs")
        .and_then(|v| v.as_object())
        .map(|m| m.len())
        .unwrap_or(0);
    assert_eq!(spec_count, 2, "Expected 2 supported specifications (ods, okf)");
    Ok(())
}

#[test]
fn test_ods_toml_workspace_configuration() -> Result<()> {
    let root = find_workspace_root()?;
    let (_, config_schema) = load_compiled_schema(CONFIG_SCHEMA)?;
    let toml_str = std::fs::read_to_string(root.join("ods.toml"))?;
    let toml_val: toml::Value = toml::from_str(&toml_str)?;
    let toml_json: serde_json::Value = serde_json::to_value(toml_val)?;

    let res = validate_instance(&config_schema, &toml_json);
    assert!(res.is_ok(), "ods.toml failed validation: {:?}", res.err());
    Ok(())
}

#[test]
fn test_positive_fixtures_pass() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema(DOC_SCHEMA)?;
    let mut positive_fixtures = find_markdown_files("tests/fixtures/valid")?;
    positive_fixtures.extend(find_markdown_files("tests/fixtures/2.0.0")?);

    assert!(!positive_fixtures.is_empty(), "No positive fixtures found");

    for fpath in positive_fixtures {
        let content = std::fs::read_to_string(&fpath)?;
        let frontmatter = extract_frontmatter(&content)?
            .unwrap_or_else(|| panic!("Missing frontmatter in {:?}", fpath));

        let res = validate_instance(&doc_schema, &frontmatter);
        assert!(
            res.is_ok(),
            "Positive fixture {:?} failed schema validation: {:?}",
            fpath.file_name().unwrap(),
            res.err()
        );
    }
    Ok(())
}

#[test]
fn test_21_positive_fixtures_pass() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema(DOC_SCHEMA_21)?;
    let fixtures = find_markdown_files("tests/fixtures/2.1.0")?;

    assert!(!fixtures.is_empty(), "No ODS 2.1 positive fixtures found");

    for fpath in fixtures {
        let content = std::fs::read_to_string(&fpath)?;
        let frontmatter = extract_frontmatter(&content)?
            .unwrap_or_else(|| panic!("Missing frontmatter in {:?}", fpath));

        let res = validate_instance(&doc_schema, &frontmatter);
        assert!(
            res.is_ok(),
            "ODS 2.1 positive fixture {:?} failed schema validation: {:?}",
            fpath.file_name().unwrap(),
            res.err()
        );
    }
    Ok(())
}

#[test]
fn test_21_unknown_predicate_rejected() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema(DOC_SCHEMA_21)?;
    let path = find_workspace_root()?.join("tests/fixtures/invalid/invalid-unknown-predicate.md");
    let content = std::fs::read_to_string(&path)?;
    let frontmatter = extract_frontmatter(&content)?.expect("frontmatter");
    let res = validate_instance(&doc_schema, &frontmatter);
    assert!(
        res.is_err(),
        "invalid-unknown-predicate.md must fail ODS 2.1 schema validation (ENUM-006)"
    );
    Ok(())
}

#[test]
fn test_21_config_fixtures() -> Result<()> {
    let root = find_workspace_root()?;
    let (_, config_schema) = load_compiled_schema(CONFIG_SCHEMA_21)?;

    let text = std::fs::read_to_string(
        root.join("tests/fixtures/config/valid-21-ontology.toml"),
    )?;
    let parsed: toml::Value = toml::from_str(&text)?;
    let result = validate_instance(&config_schema, &serde_json::to_value(parsed)?);
    assert!(
        result.is_ok(),
        "valid-21-ontology.toml must validate against 2.1 config schema: {:?}",
        result.err()
    );
    Ok(())
}

#[test]
fn test_negative_fixtures_are_rejected() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema(DOC_SCHEMA)?;
    let negative_fixtures = find_markdown_files("tests/fixtures/invalid")?;

    assert!(!negative_fixtures.is_empty(), "No negative fixtures found");

    for fpath in negative_fixtures {
        let content = std::fs::read_to_string(&fpath)?;
        if let Some(frontmatter) = extract_frontmatter(&content)? {
            let res = validate_instance(&doc_schema, &frontmatter);
            assert!(
                res.is_err(),
                "Negative fixture {:?} was expected to fail validation, but it passed!",
                fpath.file_name().unwrap()
            );
        }
    }
    Ok(())
}

#[test]
fn test_repository_spec_and_guide_documents() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema(DOC_SCHEMA)?;
    let mut prose_files = find_markdown_files("specs")?;
    prose_files.extend(find_markdown_files("guides")?);

    assert!(prose_files.len() >= 15, "Expected at least 15 spec/guide files");

    for fpath in prose_files {
        let content = std::fs::read_to_string(&fpath)?;
        if let Some(frontmatter) = extract_frontmatter(&content)? {
            let res = validate_instance(&doc_schema, &frontmatter);
            assert!(
                res.is_ok(),
                "Specification or guide document {:?} failed frontmatter schema validation: {:?}",
                fpath.file_name().unwrap(),
                res.err()
            );
        }
    }
    Ok(())
}

#[test]
fn test_no_broken_internal_links() -> Result<()> {
    let broken = find_broken_links()?;
    assert!(
        broken.is_empty(),
        "Found {} broken internal link(s):\n{}",
        broken.len(),
        broken
            .iter()
            .map(|b| format!("  {}", b))
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}

#[test]
fn test_prose_matches_schema_enums() -> Result<()> {
    let (doc_schema_json, _) = load_compiled_schema(DOC_SCHEMA)?;
    let root = find_workspace_root()?;

    for (pointer, chapter) in [
        ("/properties/status/enum", "specs/keys.md"),
        ("/properties/share/enum", "specs/keys.md"),
    ] {
        let members = schema_enum(&doc_schema_json, pointer)?;
        let tokens = prose_code_tokens(chapter)?;
        for member in &members {
            assert!(
                tokens.contains(member),
                "Enum member `{}` at {} is not documented in {}",
                member,
                pointer,
                chapter
            );
        }
    }

    let indexes = std::fs::read_to_string(root.join("specs/indexes.md"))?;
    let depth = doc_schema_json
        .pointer("/properties/context")
        .is_none();
    assert!(
        indexes.contains("default_max_depth"),
        "specs/indexes.md must document context.default_max_depth"
    );
    let _ = depth;

    Ok(())
}

#[test]
fn test_config_enums_match_prose() -> Result<()> {
    let (config_schema_json, _) = load_compiled_schema(CONFIG_SCHEMA)?;
    let indexes = std::fs::read_to_string(find_workspace_root()?.join("specs/indexes.md"))?;

    for (pointer, label) in [("/properties/dialect/enum", "dialect")] {
        for member in schema_enum(&config_schema_json, pointer)? {
            assert!(
                indexes.contains(&format!("`{}`", member)),
                "{} value `{}` is accepted by config.schema.json but not documented in specs/indexes.md",
                label,
                member
            );
        }
    }

    Ok(())
}

#[test]
fn test_default_max_depth_in_config_schema() -> Result<()> {
    let (config_schema_json, _) = load_compiled_schema(CONFIG_SCHEMA)?;
    let node = config_schema_json
        .pointer("/properties/context/properties/default_max_depth")
        .expect("context.default_max_depth must exist");

    let minimum = node.get("minimum").and_then(|v| v.as_i64()).unwrap_or(-1);
    let maximum = node.get("maximum").and_then(|v| v.as_i64()).unwrap_or(-1);
    assert_eq!(minimum, 0);
    assert_eq!(maximum, 10);

    let root = find_workspace_root()?;
    for chapter in ["specs/context.md", "specs/indexes.md"] {
        let text = std::fs::read_to_string(root.join(chapter))?;
        assert!(
            text.contains("default_max_depth") || text.contains(&format!("`{}`", maximum)),
            "{} must document default_max_depth",
            chapter
        );
    }

    Ok(())
}

#[test]
fn test_tier2_fixtures_are_schema_valid_and_declare_a_rule() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema(DOC_SCHEMA)?;
    let fixtures = find_markdown_files("tests/fixtures/tier2")?;
    assert!(!fixtures.is_empty(), "No Tier 2 fixtures found");

    for path in fixtures {
        let content = std::fs::read_to_string(&path)?;
        let frontmatter = extract_frontmatter(&content)?
            .ok_or_else(|| anyhow::anyhow!("Tier 2 fixture {:?} has no frontmatter", path))?;

        let expected = frontmatter
            .get("x-ods-expect")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                anyhow::anyhow!("Tier 2 fixture {:?} must declare 'x-ods-expect'", path)
            })?;
        assert!(
            expected.contains('-')
                && expected
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-'),
            "Tier 2 fixture {:?} declares malformed rule id {:?}",
            path,
            expected
        );

        let result = validate_instance(&doc_schema, &frontmatter);
        assert!(
            result.is_ok(),
            "Tier 2 fixture {:?} must be schema-valid (its defect is semantic): {:?}",
            path,
            result.err()
        );
    }

    Ok(())
}

#[test]
fn test_every_tier2_rule_has_a_fixture() -> Result<()> {
    let root = find_workspace_root()?;
    let validation = std::fs::read_to_string(root.join("specs/validation.md"))?;

    let mut tier2_rules: Vec<String> = Vec::new();
    for line in validation.lines() {
        if !line.contains("Tier 2") {
            continue;
        }
        let mut chars = line.chars().peekable();
        let mut current = String::new();
        let mut in_code = false;
        while let Some(c) = chars.next() {
            match c {
                '`' if !in_code => {
                    in_code = true;
                    current.clear();
                }
                '`' if in_code => {
                    in_code = false;
                    let token = current.trim();
                    let looks_like_rule = token.len() >= 5
                        && token.contains('-')
                        && token
                            .chars()
                            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-');
                    if looks_like_rule && !tier2_rules.contains(&token.to_string()) {
                        tier2_rules.push(token.to_string());
                    }
                }
                _ if in_code => current.push(c),
                _ => {}
            }
        }
    }
    assert!(
        !tier2_rules.is_empty(),
        "Could not parse any Tier 2 rule ids out of specs/validation.md"
    );

    let mut covered: Vec<String> = Vec::new();
    for path in find_markdown_files("tests/fixtures/tier2")? {
        let content = std::fs::read_to_string(&path)?;
        if let Some(fm) = extract_frontmatter(&content)? {
            if let Some(rule) = fm.get("x-ods-expect").and_then(|v| v.as_str()) {
                covered.push(rule.to_string());
            }
        }
    }

    for entry in std::fs::read_dir(root.join("tests/fixtures/config"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e == "toml") {
            for line in std::fs::read_to_string(&path)?.lines() {
                if let Some(rule) = line.trim().strip_prefix("# x-ods-expect:") {
                    covered.push(rule.trim().to_string());
                }
            }
        }
    }

    let uncovered: Vec<&String> = tier2_rules
        .iter()
        .filter(|rule| !covered.contains(rule))
        .collect();

    assert!(
        uncovered.is_empty(),
        "Tier 2 rules with no fixture in tests/fixtures/tier2/: {:?}",
        uncovered
    );

    Ok(())
}

#[test]
fn test_config_fixtures() -> Result<()> {
    let root = find_workspace_root()?;
    let (_, config_schema) = load_compiled_schema(CONFIG_SCHEMA)?;

    let to_json = |name: &str| -> Result<Value> {
        let text = std::fs::read_to_string(root.join("tests/fixtures/config").join(name))?;
        let parsed: toml::Value = toml::from_str(&text)?;
        Ok(serde_json::to_value(parsed)?)
    };

    for name in ["valid-flat.toml"] {
        let result = validate_instance(&config_schema, &to_json(name)?);
        assert!(result.is_ok(), "{} must validate: {:?}", name, result.err());
    }

    for name in [
        "invalid-spec-version.toml",
        "invalid-unknown-key.toml",
    ] {
        let result = validate_instance(&config_schema, &to_json(name)?);
        assert!(result.is_err(), "{} was expected to fail validation", name);
    }

    for path in ["ods.toml", "tests/fixtures/valid/valid-ods.toml"] {
        let text = std::fs::read_to_string(root.join(path))?;
        let parsed: toml::Value = toml::from_str(&text)?;
        let result = validate_instance(&config_schema, &serde_json::to_value(parsed)?);
        assert!(result.is_ok(), "{} must validate: {:?}", path, result.err());
    }

    Ok(())
}
