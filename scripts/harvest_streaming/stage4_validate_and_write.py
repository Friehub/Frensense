"""Stage 4: Extract changed functions, validate, write to corpus format."""
import re
import difflib
from pathlib import Path
from checkpoint import get_db

CWE_SLUG = {
    "CWE-89": "sql_injection", "CWE-78": "cmd_injection", "CWE-79": "xss",
    "CWE-22": "path_traversal", "CWE-918": "ssrf", "CWE-502": "deserialization",
    "CWE-416": "use_after_free", "CWE-476": "null_deref", "CWE-362": "race_condition",
    "CWE-190": "integer_overflow", "CWE-787": "oob_write", "CWE-617": "assertion_failure",
    "CWE-369": "divide_by_zero", "CWE-400": "resource_exhaustion", "CWE-770": "no_resource_limit",
    "CWE-94": "code_injection", "CWE-352": "csrf", "CWE-601": "open_redirect",
}


def changed_function_names(before: str, after: str, lang: str) -> set:
    fn_pattern = (
        re.compile(r"\bfn\s+(\w+)")
        if lang == "rust"
        else re.compile(r"\bfunction\s+(\w+)|(\w+)\s*\([^)]*\)\s*[:{]")
    )
    names = set()
    for src in (before, after):
        for m in fn_pattern.finditer(src):
            names.add(next(g for g in m.groups() if g))
    return names


def extract_function_source_regex(src: str, fn_name: str, lang: str) -> str | None:
    """Extract function source using regex (simpler than tree-sitter)."""
    if lang == "rust":
        pattern = re.compile(r"\bfn\s+" + re.escape(fn_name) + r"\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*\{")
    else:
        pattern = re.compile(r"\bfunction\s+" + re.escape(fn_name) + r"\s*\([^)]*\)(?:\s*:\s*[^{]+)?\s*\{")
    
    match = pattern.search(src)
    if not match:
        return None
    
    start = match.start()
    # Find matching closing brace
    brace_count = 0
    for i, c in enumerate(src[start:]):
        if c == '{':
            brace_count += 1
        elif c == '}':
            brace_count -= 1
            if brace_count == 0:
                return src[start:start + i + 1]
    return None


def validate_and_write(batch_id: int, raw_dir: Path, output_dir: Path, min_lines: int = 6):
    db = get_db()
    output_dir.mkdir(parents=True, exist_ok=True)
    written = 0

    for meta_path in raw_dir.glob("*.meta"):
        cve_id, cwe_id, lang, repo, sha = meta_path.read_text().strip().split("\t")
        stem = meta_path.stem
        ext = ".rs" if lang == "rust" else ".ts"
        before_path = raw_dir / f"{stem}{ext}.before"
        after_path = raw_dir / f"{stem}{ext}.after"
        if not before_path.exists() or not after_path.exists():
            continue

        before_src, after_src = before_path.read_text(), after_path.read_text()
        candidate_names = changed_function_names(before_src, after_src, lang)

        for fn_name in candidate_names:
            before_fn = extract_function_source_regex(before_src, fn_name, lang)
            after_fn = extract_function_source_regex(after_src, fn_name, lang)
            if not before_fn or not after_fn:
                continue
            if before_fn.strip() == after_fn.strip():
                continue
            if len(before_fn.splitlines()) < min_lines:
                continue

            pattern_name = f"{lang}_cvefixes_{CWE_SLUG.get(cwe_id, 'cve')}_{cve_id.lower()}_{fn_name}"[:100]
            pos_path = output_dir / f"{pattern_name}_positive{ext}"
            neg_path = output_dir / f"{pattern_name}_negative{ext}"
            if pos_path.exists():
                continue

            pos_path.write_text(before_fn)
            neg_path.write_text(after_fn)
            db.execute(
                "INSERT OR IGNORE INTO written_pairs (pattern_name, cve_id, batch) VALUES (?, ?, ?)",
                (pattern_name, cve_id, batch_id),
            )
            written += 1

    db.commit()
    print(f"Stage 4 done: {written} validated pairs written to {output_dir}")
    return written


if __name__ == "__main__":
    import sys
    b = int(sys.argv[1])
    validate_and_write(b, Path(f"raw/batch_{b}"), Path("corpus/targets"))
