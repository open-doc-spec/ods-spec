use anyhow::Result;
use serde_json::Value;
use ods_spec_conformance::*;

#[test]
fn test_all_schemas_compile_cleanly() -> Result<()> {
    let _ = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/config.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/profile.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/ontology.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/memory.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/attestation.schema.json")?;
    Ok(())
}

#[test]
fn test_lifecycle_metadata_coverage() -> Result<()> {
    let (doc_schema_json, _) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
    let (prop_count, spec_count) = verify_schema_lifecycle_metadata(&doc_schema_json)?;
    assert!(prop_count >= 20, "Expected at least 20 properties with lifecycle metadata, found {}", prop_count);
    assert_eq!(spec_count, 2, "Expected 2 supported specifications (ods, okf)");
    Ok(())
}

#[test]
fn test_ods_toml_workspace_configuration() -> Result<()> {
    let root = find_workspace_root()?;
    let (_, config_schema) = load_compiled_schema("schemas/1.1.0/config.schema.json")?;
    let toml_str = std::fs::read_to_string(root.join("ods.toml"))?;
    let toml_val: toml::Value = toml::from_str(&toml_str)?;
    let toml_json: serde_json::Value = serde_json::to_value(toml_val)?;

    let res = validate_instance(&config_schema, &toml_json);
    assert!(res.is_ok(), "ods.toml failed validation: {:?}", res.err());
    Ok(())
}

#[test]
fn test_positive_fixtures_pass() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
    let mut positive_fixtures = find_markdown_files("tests/fixtures/valid")?;
    positive_fixtures.extend(find_markdown_files("tests/fixtures/1.1.0")?);

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
fn test_negative_fixtures_are_rejected() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
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
    let (_, doc_schema) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
    let mut prose_files = find_markdown_files("specs")?;
    prose_files.extend(find_markdown_files("guides")?);

    assert!(prose_files.len() >= 20, "Expected at least 20 spec/guide files");

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

// ─────────────────────────────────────────────────────────────────────────────
// Spec integrity guards
// ─────────────────────────────────────────────────────────────────────────────

/// Every relative Markdown link and `#anchor` across the specification,
/// the guides, and the root documents must resolve.
///
/// Baseline when this guard was added: 24 dangling targets, 22 of them
/// glossary cross-references left behind by a chapter renumbering.
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

/// The prose and the schemas must agree on every closed vocabulary.
///
/// Each assertion below corresponds to a contradiction that was live in the
/// specification: the code-role count (8 vs 10), the memory tiers (4 vs 5),
/// and the relation predicates.
#[test]
fn test_prose_matches_schema_enums() -> Result<()> {
    let (doc_schema_json, _) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;

    // Code roles: assets.md is the canonical catalog.
    let roles = schema_enum(&doc_schema_json, "/$defs/codeBinding/properties/role/enum")?;
    let assets_tokens = prose_code_tokens("specs/assets.md")?;
    for role in &roles {
        assert!(
            assets_tokens.contains(role),
            "Code role `{}` is in document.schema.json but never documented in specs/assets.md",
            role
        );
    }
    assert!(
        std::fs::read_to_string(find_workspace_root()?.join("specs/assets.md"))?
            .contains(&format!("The {} Standard Code Roles", roles.len())),
        "specs/assets.md must state the code-role count as {}, matching the schema enum",
        roles.len()
    );

    // Memory tiers: graph.md is the canonical definition.
    let tiers = schema_enum(&doc_schema_json, "/$defs/memoryBlock/properties/tier/enum")?;
    let graph_tokens = prose_code_tokens("specs/graph.md")?;
    for tier in &tiers {
        assert!(
            graph_tokens.contains(tier),
            "Memory tier `{}` is in document.schema.json but never documented in specs/graph.md",
            tier
        );
    }
    assert!(
        std::fs::read_to_string(find_workspace_root()?.join("specs/graph.md"))?
            .contains(&format!("The {} Memory Tiers", tiers.len())),
        "specs/graph.md must state the memory-tier count as {}, matching the schema enum",
        tiers.len()
    );

    // Relation predicates: graph.md §4.1 is the canonical vocabulary.
    let predicates = schema_enum(
        &doc_schema_json,
        "/$defs/semanticRelation/oneOf/1/properties/predicate/enum",
    )?;
    for predicate in &predicates {
        assert!(
            graph_tokens.contains(predicate),
            "Relation predicate `{}` is in document.schema.json but never documented in specs/graph.md",
            predicate
        );
    }

    // Statuses and share levels.
    for (pointer, chapter) in [
        ("/$defs/odsEngine/properties/status/enum", "specs/keys.md"),
        ("/$defs/odsEngine/properties/share/enum", "specs/keys.md"),
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

    Ok(())
}

/// `service.mode` in `config.schema.json` must match what `indexes.md` tells
/// authors to write. A documented value that fails its own schema is worse
/// than no documentation.
#[test]
fn test_config_enums_match_prose() -> Result<()> {
    let (config_schema_json, _) = load_compiled_schema("schemas/1.1.0/config.schema.json")?;
    let indexes = std::fs::read_to_string(find_workspace_root()?.join("specs/indexes.md"))?;

    for (pointer, label) in [
        ("/properties/service/properties/mode/enum", "service.mode"),
        ("/properties/dialect/enum", "dialect"),
        (
            "/properties/memory/properties/backend/enum",
            "memory.backend",
        ),
    ] {
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

/// `max-depth` bounds must be stated identically in the schema and in the two
/// chapters that describe traversal. They previously read "none", "5", and "10".
#[test]
fn test_max_depth_bounds_are_consistent() -> Result<()> {
    let (doc_schema_json, _) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
    let node = doc_schema_json
        .pointer("/$defs/contextConfig/properties/max-depth")
        .expect("contextConfig.max-depth must exist");

    let minimum = node.get("minimum").and_then(|v| v.as_i64()).unwrap_or(-1);
    let maximum = node.get("maximum").and_then(|v| v.as_i64()).unwrap_or(-1);
    assert_eq!(minimum, 0, "max-depth minimum must be 0");

    let root = find_workspace_root()?;
    let bound = format!("`{}`", maximum);
    for chapter in ["specs/keys.md", "specs/glossary.md"] {
        let text = std::fs::read_to_string(root.join(chapter))?;
        assert!(
            text.contains(&bound) || text.contains(&format!("0`–`{}", maximum)),
            "{} must state the max-depth ceiling of {}",
            chapter,
            maximum
        );
    }

    Ok(())
}

/// Every deprecated key must carry a complete lifecycle record, so tooling can
/// surface the removal timeline without parsing prose.
#[test]
fn test_deprecated_keys_declare_a_removal_target() -> Result<()> {
    for schema in [
        "schemas/1.1.0/document.schema.json",
        "schemas/1.1.0/config.schema.json",
    ] {
        let (schema_json, _) = load_compiled_schema(schema)?;
        let mut found = 0;

        let mut stack = vec![&schema_json];
        while let Some(node) = stack.pop() {
            if let Some(lifecycle) = node.get("x-ods-lifecycle") {
                if lifecycle.get("status").and_then(|s| s.as_str()) == Some("deprecated") {
                    found += 1;
                    for field in ["deprecated_in", "removed_in", "spec"] {
                        assert!(
                            lifecycle.get(field).is_some(),
                            "A deprecated key in {} is missing '{}' in its x-ods-lifecycle record",
                            schema,
                            field
                        );
                    }
                }
            }
            match node {
                Value::Object(map) => stack.extend(map.values()),
                Value::Array(items) => stack.extend(items.iter()),
                _ => {}
            }
        }

        assert!(
            found > 0,
            "{} declares no deprecated keys; the 1.1 deprecations should be annotated there",
            schema
        );
    }

    Ok(())
}

/// Tier 2 rules are workspace-semantic: no JSON Schema can express "this
/// `depends` chain forms a cycle" or "this entity name is declared twice".
///
/// The fixtures under `tests/fixtures/tier2/` are the executable documentation
/// for those rules. Each declares the rule it is designed to trigger via
/// `x-ods-expect`, so an implementer building a Graph-profile linter has a
/// concrete case per rule ID. They MUST remain schema-valid — the defect they
/// carry is semantic, not structural.
#[test]
fn test_tier2_fixtures_are_schema_valid_and_declare_a_rule() -> Result<()> {
    let (_, doc_schema) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
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
            expected.contains('-') && expected.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-'),
            "Tier 2 fixture {:?} declares malformed rule id {:?}",
            path,
            expected
        );

        // The defect must be semantic, not structural: these files are
        // well-formed ODS that only a workspace-aware linter can reject.
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

/// Every Tier 2 rule listed in the validation matrix must have at least one
/// fixture. Adding a rule to `specs/validation.md` without a fixture fails here.
#[test]
fn test_every_tier2_rule_has_a_fixture() -> Result<()> {
    let root = find_workspace_root()?;
    let validation = std::fs::read_to_string(root.join("specs/validation.md"))?;

    // Rule ids appear as `PREFIX-00N` in the matrix; Tier 2 rows say so explicitly.
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
                    let _ = chars.peek();
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

    // Workspace-level rules are demonstrated by an `ods.toml` fixture instead,
    // which declares its rule id in a leading comment.
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

/// Both documented `ods.toml` dialects must validate, and malformed configs
/// must be rejected. Previously only the repository's own `ods.toml` was checked,
/// so the nested form in the fixtures was never exercised at all.
#[test]
fn test_config_fixtures() -> Result<()> {
    let root = find_workspace_root()?;
    let (_, config_schema) = load_compiled_schema("schemas/1.1.0/config.schema.json")?;

    let to_json = |name: &str| -> Result<Value> {
        let text = std::fs::read_to_string(root.join("tests/fixtures/config").join(name))?;
        let parsed: toml::Value = toml::from_str(&text)?;
        Ok(serde_json::to_value(parsed)?)
    };

    for name in ["valid-flat.toml", "valid-nested-deprecated.toml"] {
        let result = validate_instance(&config_schema, &to_json(name)?);
        assert!(result.is_ok(), "{} must validate: {:?}", name, result.err());
    }

    for name in [
        "invalid-spec-version.toml",
        "invalid-service-mode.toml",
        "invalid-unknown-key.toml",
    ] {
        let result = validate_instance(&config_schema, &to_json(name)?);
        assert!(result.is_err(), "{} was expected to fail validation", name);
    }

    // The repository's own workspace file and the legacy fixture must both pass.
    for path in ["ods.toml", "tests/fixtures/valid/valid-ods.toml"] {
        let text = std::fs::read_to_string(root.join(path))?;
        let parsed: toml::Value = toml::from_str(&text)?;
        let result = validate_instance(&config_schema, &serde_json::to_value(parsed)?);
        assert!(result.is_ok(), "{} must validate: {:?}", path, result.err());
    }

    Ok(())
}
