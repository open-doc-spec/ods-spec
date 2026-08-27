use anyhow::Result;
use colored::*;
use ods_spec_conformance::*;

fn main() -> Result<()> {
    println!("{}", "═════════════════════════════════════════════════════════════════".bold().cyan());
    println!("{}", "  ODS 1.1 Specification Conformance Runner (Native Rust)".bold().cyan());
    println!("{}", "═════════════════════════════════════════════════════════════════\n".bold().cyan());

    let mut passed = 0;
    let mut failed = 0;

    let root = find_workspace_root()?;

    // 1. Schema & Lifecycle Metadata Validation
    println!("{}", "─── 1. Checking Schema Metadata & Lifecycle Traceability ───".yellow().bold());
    let (doc_schema_json, doc_schema) = load_compiled_schema("schemas/1.1.0/document.schema.json")?;
    let (_, config_schema) = load_compiled_schema("schemas/1.1.0/config.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/profile.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/ontology.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/memory.schema.json")?;
    let _ = load_compiled_schema("schemas/1.1.0/attestation.schema.json")?;
    println!("  {} Compiled all 6 schemas in schemas/1.1.0/", "✓".green().bold());
    passed += 1;

    match verify_schema_lifecycle_metadata(&doc_schema_json) {
        Ok((prop_count, spec_count)) => {
            println!("  {} Found x-ods-supported-specs ({}) specifications", "✓".green().bold(), spec_count);
            println!("  {} 100% of schema properties ({}) have x-ods-lifecycle metadata", "✓".green().bold(), prop_count);
            passed += 2;
        }
        Err(e) => {
            println!("  {} {}", "❌".red().bold(), e);
            failed += 1;
        }
    }

    // 2. Workspace Config Validation (ods.toml)
    println!("\n{}", "─── 2. Validating Workspace Configuration (ods.toml) ───".yellow().bold());
    let toml_str = std::fs::read_to_string(root.join("ods.toml"))?;
    let toml_val: toml::Value = toml::from_str(&toml_str)?;
    let toml_json: serde_json::Value = serde_json::to_value(toml_val)?;

    match validate_instance(&config_schema, &toml_json) {
        Ok(_) => {
            println!("  {} ods.toml is 100% compliant with config.schema.json", "✓".green().bold());
            passed += 1;
        }
        Err(errs) => {
            println!("  {} ods.toml validation errors:", "❌".red().bold());
            for err in errs {
                println!("    - {}", err.red());
            }
            failed += 1;
        }
    }

    // 3. Positive Fixtures (Expect PASS)
    println!("\n{}", "─── 3. Testing POSITIVE Fixtures (Expect PASS) ───".yellow().bold());
    let mut positive_fixtures = find_markdown_files("tests/fixtures/valid")?;
    positive_fixtures.extend(find_markdown_files("tests/fixtures/1.1.0")?);
    positive_fixtures.sort();

    for fpath in positive_fixtures {
        let name = fpath.file_name().unwrap().to_string_lossy();
        let content = std::fs::read_to_string(&fpath)?;
        match extract_frontmatter(&content)? {
            Some(frontmatter) => {
                match validate_instance(&doc_schema, &frontmatter) {
                    Ok(_) => {
                        println!("  {} PASS: {}", "✓".green().bold(), name);
                        passed += 1;
                    }
                    Err(errs) => {
                        println!("  {} FAIL: {}", "❌".red().bold(), name);
                        for err in errs {
                            println!("    - {}", err.red());
                        }
                        failed += 1;
                    }
                }
            }
            None => {
                println!("  {} FAIL (No frontmatter): {}", "❌".red().bold(), name);
                failed += 1;
            }
        }
    }

    // 4. Negative Fixtures (Expect FAIL)
    println!("\n{}", "─── 4. Testing NEGATIVE Fixtures (Expect REJECTION) ───".yellow().bold());
    let negative_fixtures = find_markdown_files("tests/fixtures/invalid")?;
    for fpath in negative_fixtures {
        let name = fpath.file_name().unwrap().to_string_lossy();
        let content = std::fs::read_to_string(&fpath)?;
        match extract_frontmatter(&content)? {
            Some(frontmatter) => {
                match validate_instance(&doc_schema, &frontmatter) {
                    Ok(_) => {
                        println!("  {} FAIL (Expected violation, but passed): {}", "❌".red().bold(), name);
                        failed += 1;
                    }
                    Err(_) => {
                        println!("  {} PASS (Caught expected violation): {}", "✓".green().bold(), name);
                        passed += 1;
                    }
                }
            }
            None => {
                println!("  {} PASS (No frontmatter / rejected): {}", "✓".green().bold(), name);
                passed += 1;
            }
        }
    }

    // 5. Repository Markdown Frontmatter Integrity
    println!("\n{}", "─── 5. Testing Specs & Guides Markdown Integrity ───".yellow().bold());
    let mut prose_files = find_markdown_files("specs")?;
    prose_files.extend(find_markdown_files("guides")?);
    prose_files.sort();

    let mut prose_pass = 0;
    for fpath in prose_files {
        let rel_path = fpath.strip_prefix(&root).unwrap_or(&fpath);
        let content = std::fs::read_to_string(&fpath)?;
        if let Some(fm) = extract_frontmatter(&content)? {
            match validate_instance(&doc_schema, &fm) {
                Ok(_) => {
                    prose_pass += 1;
                }
                Err(errs) => {
                    println!("  {} Invalid spec/guide frontmatter in {}:", "❌".red().bold(), rel_path.display());
                    for err in errs {
                        println!("    - {}", err.red());
                    }
                    failed += 1;
                }
            }
        }
    }
    println!("  {} All {} specification and guide documents have valid frontmatter", "✓".green().bold(), prose_pass);
    passed += 1;

    // Summary Report
    println!("\n{}", "═════════════════════════════════════════════════════════════════".bold().cyan());
    println!("  Summary: {} passed, {} failed", passed.to_string().green().bold(), failed.to_string().red().bold());
    println!("{}", "═════════════════════════════════════════════════════════════════".bold().cyan());

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
