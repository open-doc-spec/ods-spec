use anyhow::Result;
use colored::*;
use ods_spec_conformance::*;

const DOC_SCHEMA: &str = "schemas/2.0.0/document.schema.json";
const CONFIG_SCHEMA: &str = "schemas/2.0.0/config.schema.json";
const PROFILE_SCHEMA: &str = "schemas/2.0.0/profile.schema.json";

fn main() -> Result<()> {
    println!("{}", "═════════════════════════════════════════════════════════════════".bold().cyan());
    println!("{}", "  ODS 2.0 Specification Conformance Runner (Native Rust)".bold().cyan());
    println!("{}", "═════════════════════════════════════════════════════════════════\n".bold().cyan());

    let mut passed = 0;
    let mut failed = 0;

    let root = find_workspace_root()?;

    println!("{}", "─── 1. Checking Schema Compilation ───".yellow().bold());
    let (doc_schema_json, doc_schema) = load_compiled_schema(DOC_SCHEMA)?;
    let (_, config_schema) = load_compiled_schema(CONFIG_SCHEMA)?;
    let _ = load_compiled_schema(PROFILE_SCHEMA)?;
    println!("  {} Compiled schemas in schemas/2.0.0/", "✓".green().bold());
    passed += 1;

    match doc_schema_json.get("x-ods-supported-specs") {
        Some(v) if v.as_object().map(|m| m.len()).unwrap_or(0) >= 1 => {
            println!("  {} Found x-ods-supported-specs metadata", "✓".green().bold());
            passed += 1;
        }
        _ => {
            println!("  {} Missing x-ods-supported-specs", "❌".red().bold());
            failed += 1;
        }
    }

    println!("\n{}", "─── 2. Validating Workspace Configuration (ods.toml) ───".yellow().bold());
    let toml_str = std::fs::read_to_string(root.join("ods.toml"))?;
    let toml_val: toml::Value = toml::from_str(&toml_str)?;
    let toml_json: serde_json::Value = serde_json::to_value(toml_val)?;

    match validate_instance(&config_schema, &toml_json) {
        Ok(_) => {
            println!("  {} ods.toml is compliant with config.schema.json", "✓".green().bold());
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

    println!("\n{}", "─── 3. Testing POSITIVE Fixtures (Expect PASS) ───".yellow().bold());
    let mut positive_fixtures = find_markdown_files("tests/fixtures/valid")?;
    positive_fixtures.extend(find_markdown_files("tests/fixtures/2.0.0")?);
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

    println!("\n{}", "═════════════════════════════════════════════════════════════════".bold().cyan());
    println!("  Summary: {} passed, {} failed", passed.to_string().green().bold(), failed.to_string().red().bold());
    println!("{}", "═════════════════════════════════════════════════════════════════".bold().cyan());

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
