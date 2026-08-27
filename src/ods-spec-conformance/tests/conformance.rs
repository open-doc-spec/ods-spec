use anyhow::Result;
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
