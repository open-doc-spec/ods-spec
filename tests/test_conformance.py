#!/usr/bin/env python3
import glob
import json
import os
import subprocess
import sys
import tempfile
import yaml
import datetime

def json_serial(obj):
    if isinstance(obj, (datetime.date, datetime.datetime)):
        return obj.isoformat()
    raise TypeError(f"Type {type(obj)} not serializable")

def test_fixtures():
    schema_path = "schemas/1.0.0/document.schema.json"
    valid_fixtures = sorted(glob.glob("tests/fixtures/valid/*.md"))
    invalid_fixtures = sorted(glob.glob("tests/fixtures/invalid/*.md"))
    
    passed_tests = 0
    failed_tests = 0

    print("═════════════════════════════════════════════════════════════════")
    print("  Running ODS Conformance Test Suite (JSON Schema Draft 2020-12)")
    print("═════════════════════════════════════════════════════════════════\n")

    print("─── Testing POSITIVE Fixtures (Expect PASS) ───")
    for fpath in valid_fixtures:
        with open(fpath, "r") as f:
            content = f.read()
        parts = content.split("---", 2)
        if len(parts) < 3:
            print(f"❌ FAIL: {fpath} (missing frontmatter)")
            failed_tests += 1
            continue
        
        data = yaml.safe_load(parts[1])
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as tmp:
            json.dump(data, tmp, default=json_serial)
            tmp_name = tmp.name

        cmd = [
            "npx", "--yes", "ajv-cli", "validate",
            "-s", schema_path,
            "-d", tmp_name,
            "--spec=draft2020",
            "--errors=text"
        ]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        os.unlink(tmp_name)

        if res.returncode == 0:
            print(f"  ✓ PASS: {os.path.basename(fpath)}")
            passed_tests += 1
        else:
            print(f"  ❌ FAIL: {os.path.basename(fpath)}")
            print("    " + res.stdout.strip())
            print("    " + res.stderr.strip())
            failed_tests += 1

    print("\n─── Testing NEGATIVE Fixtures (Expect FAIL) ───")
    for fpath in invalid_fixtures:
        with open(fpath, "r") as f:
            content = f.read()
        parts = content.split("---", 2)
        if len(parts) < 3:
            print(f"❌ FAIL: {fpath} (missing frontmatter)")
            failed_tests += 1
            continue
        
        data = yaml.safe_load(parts[1])
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as tmp:
            json.dump(data, tmp, default=json_serial)
            tmp_name = tmp.name

        cmd = [
            "npx", "--yes", "ajv-cli", "validate",
            "-s", schema_path,
            "-d", tmp_name,
            "--spec=draft2020",
            "--errors=text"
        ]
        res = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        os.unlink(tmp_name)

        if res.returncode != 0:
            print(f"  ✓ PASS (Caught expected violation): {os.path.basename(fpath)}")
            passed_tests += 1
        else:
            print(f"  ❌ FAIL (Expected error, but passed): {os.path.basename(fpath)}")
            failed_tests += 1

    print("\n═════════════════════════════════════════════════════════════════")
    print(f"  Summary: {passed_tests} passed, {failed_tests} failed")
    print("═════════════════════════════════════════════════════════════════")

    if failed_tests > 0:
        sys.exit(1)

if __name__ == "__main__":
    test_fixtures()
