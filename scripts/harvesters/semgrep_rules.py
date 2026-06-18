#!/usr/bin/env python3
"""
Semgrep Rules → FrenSense Corpus Converter

Parses semgrep YAML rules and extracts pattern information to generate
corpus pairs. The semgrep pattern describes what the vulnerable code looks
like; we generate a negative (fixed) example from the rule's metadata.

Usage:
    python3 scripts/harvesters/semgrep_rules.py --rules-dir /tmp/semgrep-rules --limit 200
"""

import argparse
import re
from pathlib import Path
from typing import Optional

try:
    import yaml
    HAS_YAML = True
except ImportError:
    HAS_YAML = False


# Known fix patterns for common semgrep pattern types
FIX_PATTERNS = {
    # SQL injection
    "pattern: $DB.query(`...`)": "Use parameterized queries: $DB.query('SELECT ... WHERE id = $1', [id])",
    "pattern: $DB.query($QUERY + $INPUT)": "Use parameterized queries instead of string concatenation",
    "pattern: $X = `...${$INPUT}...`": "Avoid template literals with user input in queries",
    # Command injection
    "pattern: exec($INPUT)": "Use execFile with args array instead of exec with shell string",
    "pattern: shell.exec($INPUT)": "Validate input against allowlist before execution",
    # XSS
    "pattern: innerHTML = $INPUT": "Use textContent instead of innerHTML",
    "pattern: document.write($INPUT)": "Use DOM manipulation APIs instead of document.write",
    # Path traversal
    "pattern: readFileSync($INPUT)": "Validate and normalize path before reading",
    # SSRF
    "pattern: fetch($INPUT)": "Validate URL against allowlist before fetching",
    # Prototype pollution
    "pattern: $OBJ[$KEY] = $VALUE": "Filter __proto__, constructor, prototype keys",
    "pattern: Object.assign($OBJ, $INPUT)": "Validate keys before merging",
    # Insecure crypto
    "pattern: md5($INPUT)": "Use sha256 or bcrypt instead of md5",
    "pattern: sha1($INPUT)": "Use sha256 or bcrypt instead of sha1",
    # Hardcoded secrets
    "pattern: $X = \"sk_live_...\"": "Use environment variables for secrets",
    "pattern: $X = \"AKIA...\"": "Use IAM roles or environment variables for AWS keys",
}


def slugify(name: str) -> str:
    return re.sub(r"[^a-z0-9]", "_", name.lower()).strip("_")[:40]


def parse_semgrep_rule(yaml_path: Path) -> Optional[dict]:
    """Parse a semgrep YAML rule and extract useful information."""
    if not HAS_YAML:
        return None

    try:
        with open(yaml_path) as f:
            data = yaml.safe_load(f)
    except Exception:
        return None

    if not data or "rules" not in data:
        return None

    rules = data["rules"]
    if not isinstance(rules, list):
        return None

    results = []
    for rule in rules:
        rule_id = rule.get("id", "")
        message = rule.get("message", "")
        severity = rule.get("severity", "WARNING")
        languages = rule.get("languages", [])
        metadata = rule.get("metadata", {})
        cwe = metadata.get("cwe", [])
        category = metadata.get("category", "")

        # Extract patterns
        patterns = rule.get("patterns", [])
        pattern_either = rule.get("pattern-either", [])
        pattern = rule.get("pattern", "")

        # Get the vulnerable pattern description
        pattern_desc = ""
        if pattern:
            pattern_desc = str(pattern)
        elif pattern_either:
            pattern_desc = " | ".join(str(p) for p in pattern_either[:3])
        elif patterns:
            for p in patterns:
                if isinstance(p, dict) and "pattern" in p:
                    pattern_desc = str(p["pattern"])
                    break

        if not pattern_desc:
            continue

        # Determine language
        lang = "typescript"
        for l in languages:
            if l in ("ts", "typescript", "tsx"):
                lang = "typescript"
                break
            elif l in ("js", "javascript", "jsx"):
                lang = "typescript"  # treat JS as TS for corpus
                break
            elif l in ("py", "python"):
                lang = "python"
                break
            elif l in ("rust", "rs"):
                lang = "rust"
                break

        # Skip non-target languages
        if lang not in ("rust", "typescript"):
            continue

        # Get CWE info
        cwe_id = ""
        cwe_desc = ""
        if cwe:
            if isinstance(cwe, list) and cwe:
                cwe_text = str(cwe[0])
                cwe_id = cwe_text.split(":")[0].strip() if ":" in cwe_text else cwe_text
                cwe_desc = cwe_text.split(":", 1)[1].strip() if ":" in cwe_text else ""
            elif isinstance(cwe, str):
                cwe_id = cwe

        # Get fix recommendation
        fix = ""
        if "fix" in rule:
            fix = str(rule["fix"])
        elif "fix-regex" in rule:
            fix = str(rule.get("fix-regex", {}).get("regex", ""))
        elif pattern_desc in FIX_PATTERNS:
            fix = FIX_PATTERNS[pattern_desc]

        results.append({
            "id": rule_id,
            "message": message,
            "severity": severity,
            "lang": lang,
            "cwe_id": cwe_id,
            "cwe_desc": cwe_desc,
            "category": category,
            "pattern": pattern_desc,
            "fix": fix,
        })

    return results


def generate_corpus_pair(rule: dict) -> Optional[dict]:
    """Generate a positive/negative corpus pair from a semgrep rule."""
    rule_id = rule["id"]
    lang = rule["lang"]
    cwe = rule["cwe_id"] or "security"

    # Clean pattern for code generation
    pattern = rule["pattern"]
    message = rule["message"]

    # Generate a minimal positive example (vulnerable code)
    pos_code = _generate_positive(pattern, message, lang)
    if not pos_code:
        return None

    # Generate a minimal negative example (fixed code)
    neg_code = _generate_negative(pattern, message, rule.get("fix", ""), lang)
    if not neg_code:
        return None

    if pos_code.strip() == neg_code.strip():
        return None

    cwe_slug = slugify(cwe) if cwe else "security"
    rule_slug = slugify(rule_id)

    return {
        "name": f"{lang}_semgrep_{cwe_slug}_{rule_slug}",
        "language": lang,
        "positive": pos_code,
        "negative": neg_code,
    }


def _generate_positive(pattern: str, message: str, lang: str) -> Optional[str]:
    """Generate vulnerable code from a semgrep pattern."""
    # Extract the core pattern
    pat = pattern.strip()

    if lang == "typescript":
        if "innerHTML" in pat:
            return "// Vulnerable: XSS via innerHTML\ndocument.getElementById('output').innerHTML = userInput;\n"
        if "exec(" in pat:
            return "// Vulnerable: command injection\nconst { exec } = require('child_process');\nexec(userInput);\n"
        if "eval(" in pat:
            return "// Vulnerable: code injection\neval(userInput);\n"
        if "fetch(" in pat and "$INPUT" in pat:
            return "// Vulnerable: SSRF\nfetch(userInput).then(r => r.text());\n"
        if "query(" in pat or "execute(" in pat:
            return "// Vulnerable: SQL injection\nconst query = `SELECT * FROM users WHERE id = ${userId}`;\ndb.query(query);\n"
        if "readFileSync(" in pat:
            return "// Vulnerable: path traversal\nconst fs = require('fs');\nconst data = fs.readFileSync(userPath);\n"
        if "Object.assign" in pat or "[$KEY]" in pat:
            return "// Vulnerable: prototype pollution\nObject.assign(target, userInput);\n"
        if "md5(" in pat or "sha1(" in pat:
            return "// Vulnerable: weak crypto\nconst hash = md5(password);\n"
        if "sk_live_" in pat or "AKIA" in pat:
            return '// Vulnerable: hardcoded secret\nconst API_KEY = "sk_live_abc123def456";\n'
        if "console.log" in pat:
            return "// Vulnerable: credential logging\nconsole.log('Auth token:', authToken);\n"
        if "JSON.parse" in pat and "untrusted" in message.lower():
            return "// Vulnerable: untrusted deserialization\nconst data = JSON.parse(untrustedInput);\n"
        if "ws://" in pat:
            return "// Vulnerable: insecure transport\nconst ws = new WebSocket('ws://example.com');\n"
        if "http://" in pat:
            return "// Vulnerable: insecure transport\nfetch('http://api.example.com/data');\n"
        # Generic fallback
        return f"// Vulnerable: {message}\n// Pattern: {pat}\nfunction vulnerable() {{\n  // TODO: implement pattern match\n}}\n"

    elif lang == "rust":
        if "unwrap()" in pat:
            return "// Vulnerable: panic on error\nlet val = some_operation().unwrap();\n"
        if "panic!" in pat:
            return "// Vulnerable: panic in library code\npanic!(\"error occurred\");\n"
        if "unsafe" in pat:
            return "// Vulnerable: unsafe block\nunsafe {{ *ptr = value; }}\n"
        if "format!" in pat and ("SELECT" in pat or "INSERT" in pat):
            return "// Vulnerable: SQL injection\nlet query = format!(\"SELECT * FROM users WHERE id = {}\", user_id);\ndb.execute(&query);\n"
        # Generic fallback
        return f"// Vulnerable: {message}\n// Pattern: {pat}\nfn vulnerable() {{\n    // TODO: implement pattern match\n}}\n"

    return None


def _generate_negative(pattern: str, message: str, fix: str, lang: str) -> Optional[str]:
    """Generate fixed code from a semgrep pattern."""
    pat = pattern.strip()

    if lang == "typescript":
        if "innerHTML" in pat:
            return "// Fixed: use textContent\ndocument.getElementById('output').textContent = userInput;\n"
        if "exec(" in pat:
            return "// Fixed: use execFile with args array\nconst { execFile } = require('child_process');\nexecFile('cmd', ['/c', 'echo', userInput]);\n"
        if "eval(" in pat:
            return "// Fixed: avoid eval\nconst result = Function('return ' + userInput)();\n"
        if "fetch(" in pat and "$INPUT" in pat:
            return "// Fixed: validate URL\nconst url = new URL(userInput);\nif (ALLOWED_HOSTS.includes(url.hostname)) {{\n  fetch(url).then(r => r.text());\n}}\n"
        if "query(" in pat or "execute(" in pat:
            return "// Fixed: parameterized query\ndb.query('SELECT * FROM users WHERE id = $1', [userId]);\n"
        if "readFileSync(" in pat:
            return "// Fixed: validate path\nconst path = require('path');\nconst resolved = path.resolve(BASE_DIR, userPath);\nif (resolved.startsWith(BASE_DIR)) {{\n  const data = fs.readFileSync(resolved);\n}}\n"
        if "Object.assign" in pat or "[$KEY]" in pat:
            return "// Fixed: filter dangerous keys\nconst safeKeys = Object.keys(input).filter(k => !['__proto__', 'constructor', 'prototype'].includes(k));\nfor (const key of safeKeys) {{ target[key] = input[key]; }}\n"
        if "md5(" in pat or "sha1(" in pat:
            return "// Fixed: use strong crypto\nconst bcrypt = require('bcrypt');\nconst hash = await bcrypt.hash(password, 10);\n"
        if "sk_live_" in pat or "AKIA" in pat:
            return '// Fixed: use environment variable\nconst API_KEY = process.env.API_KEY;\nif (!API_KEY) throw new Error("API_KEY required");\n'
        if "console.log" in pat:
            return "// Fixed: use structured logging\nlogger.info({ event: 'auth', userId: user.id });\n"
        if "ws://" in pat:
            return "// Fixed: use secure transport\nconst ws = new WebSocket('wss://example.com');\n"
        if "http://" in pat:
            return "// Fixed: use HTTPS\nfetch('https://api.example.com/data');\n"
        # Always return a negative — never None
        if fix:
            return f"// Fixed: {fix}\nfunction safe() {{\n  // {fix}\n}}\n"
        return f"// Fixed: {message}\n// Apply appropriate sanitization\nfunction safe() {{\n  // TODO: implement fix\n}}\n"

    elif lang == "rust":
        if "unwrap()" in pat:
            return "// Fixed: propagate error\nlet val = some_operation()?;\n"
        if "panic!" in pat:
            return "// Fixed: return error\nreturn Err(MyError::new(\"error occurred\"));\n"
        if "unsafe" in pat:
            return "// Fixed: safe alternative\nif let Some(val) = ptr.as_ref() {{\n    // use val safely\n}}\n"
        if "format!" in pat and ("SELECT" in pat or "INSERT" in pat):
            return "// Fixed: parameterized query\ndb.execute(\"SELECT * FROM users WHERE id = $1\", &[&user_id])?;\n"
        # Always return a negative — never None
        if fix:
            return f"// Fixed: {fix}\nfn safe() -> Result<(), Error> {{\n    // {fix}\n    Ok(())\n}}\n"
        return f"// Fixed: {message}\nfn safe() -> Result<(), Error> {{\n    // TODO: implement fix\n    Ok(())\n}}\n"

    # Always return a negative for any language
    return f"// Fixed: {message}\n// Apply appropriate fix\n"


def harvest_semgrep_rules(
    rules_dir: Path,
    output_dir: Path,
    limit: int,
    dry_run: bool,
) -> int:
    """Harvest corpus pairs from semgrep rules."""
    if not HAS_YAML:
        print("  Semgrep: 'pyyaml' package required")
        return 0

    harvested = 0
    yaml_files = sorted(rules_dir.rglob("*.yaml"))

    for yaml_path in yaml_files:
        if harvested >= limit:
            break

        rules = parse_semgrep_rule(yaml_path)
        if not rules:
            continue

        for rule in rules:
            if harvested >= limit:
                break

            pair = generate_corpus_pair(rule)
            if not pair:
                continue

            ext = "ts" if pair["language"] == "typescript" else "rs"
            name = pair["name"]

            if dry_run:
                print(f"  Would write: {name}")
                harvested += 1
                continue

            (output_dir / f"{name}_positive.{ext}").write_text(pair["positive"])
            (output_dir / f"{name}_negative.{ext}").write_text(pair["negative"])
            harvested += 1

    return harvested


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--rules-dir", default="/tmp/semgrep-rules")
    parser.add_argument("--limit", type=int, default=200)
    parser.add_argument("--output", default="corpus/targets")
    parser.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()

    rules_dir = Path(args.rules_dir)
    if not rules_dir.exists():
        print(f"Rules directory not found: {rules_dir}")
        return 1

    output_dir = Path(args.output)
    output_dir.mkdir(parents=True, exist_ok=True)

    count = harvest_semgrep_rules(rules_dir, output_dir, args.limit, args.dry_run)
    print(f"\nHarvested: {count} patterns from semgrep rules")
    return 0


if __name__ == "__main__":
    exit(main())
