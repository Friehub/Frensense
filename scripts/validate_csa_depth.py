#!/usr/bin/env python3
"""
Validate CSA (Contract Surface Analysis) pattern depth.

Ensures CSA patterns meet the quality criteria from the document:
- Property 1: Signature similarity > 0.85 (tests contract violation, not naming)
- Property 2: At least 1 conditional in positive (realistic branching depth)
- Property 3: Line count >= 12 (forces realistic context)
- Property 4: Branch with no rejecting path (fake-looking but real branch)

Usage:
    python3 scripts/validate_csa_depth.py corpus/targets/ts_csa_*.ts
    python3 scripts/validate_csa_depth.py --all
"""

import argparse
import re
import sys
from pathlib import Path


def count_lines(filepath: Path) -> int:
    """Count non-empty, non-comment lines in a file."""
    try:
        content = filepath.read_text()
        lines = content.split('\n')
        count = 0
        for line in lines:
            stripped = line.strip()
            if stripped and not stripped.startswith('//') and not stripped.startswith('/*'):
                count += 1
        return count
    except Exception:
        return 0


def count_conditionals(filepath: Path) -> int:
    """Count conditional statements (if/else/switch/match) in a file."""
    try:
        content = filepath.read_text()
        # Count if/else if/else/switch/match patterns
        patterns = [
            r'\bif\b',
            r'\belse\b',
            r'\bswitch\b',
            r'\bmatch\b',
            r'\bcase\b',
        ]
        count = 0
        for pattern in patterns:
            count += len(re.findall(pattern, content))
        return count
    except Exception:
        return 0


def extract_signatures(filepath: Path) -> list[str]:
    """Extract function signatures from a file."""
    try:
        content = filepath.read_text()
        # Match function declarations: function name(params): type
        sig_patterns = [
            r'function\s+(\w+)\s*\([^)]*\)',  # TypeScript/JavaScript
            r'fn\s+(\w+)\s*\([^)]*\)',  # Rust
        ]
        signatures = []
        for pattern in sig_patterns:
            matches = re.findall(pattern, content)
            signatures.extend(matches)
        return signatures
    except Exception:
        return []


def signature_similarity(sig1: str, sig2: str) -> float:
    """Compute similarity between two function names (simplified)."""
    if sig1 == sig2:
        return 1.0
    # Simple Jaccard on character sets
    set1 = set(sig1)
    set2 = set(sig2)
    intersection = len(set1 & set2)
    union = len(set1 | set2)
    return intersection / union if union > 0 else 0.0


def has_branch_with_no_rejecting_path(filepath: Path) -> bool:
    """
    Check if file has a branch that looks like a check but doesn't reject.
    Heuristic: look for patterns like:
    - if (condition) { console.warn(...); }  (logs but doesn't return/reject)
    - if (condition) { console.log(...); }   (logs but doesn't return/reject)
    - if condition { println!(...); }        (Rust logging without rejection)
    """
    try:
        content = filepath.read_text()
        # Look for if blocks that only contain logging (no return/throw/reject)
        if_blocks = re.findall(r'if\s*\([^)]*\)\s*\{([^}]+)\}', content)
        for block in if_blocks:
            # Check if block contains logging but no rejection
            has_logging = bool(re.search(r'console\.(warn|log|info)', block))
            has_rejection = bool(re.search(r'\b(return|throw|reject|exit)\b', block))
            if has_logging and not has_rejection:
                return True
        
        # Also check Rust-style if blocks
        rust_if_blocks = re.findall(r'if\s+[^{]+\{([^}]+)\}', content)
        for block in rust_if_blocks:
            # Check if block contains println! but no return/Err
            has_logging = bool(re.search(r'println!', block))
            has_rejection = bool(re.search(r'\b(return|Err\(|panic!)\b', block))
            if has_logging and not has_rejection:
                return True
        
        return False
    except Exception:
        return False


def validate_csa_pair(positive_path: Path, negative_path: Path) -> list[str]:
    """
    Validate a CSA pair meets quality criteria.
    
    Returns list of error messages (empty if valid).
    """
    errors = []
    
    # Property 1: Check function signatures exist and are similar
    pos_sigs = extract_signatures(positive_path)
    neg_sigs = extract_signatures(negative_path)
    
    if not pos_sigs:
        errors.append("No function signatures found in positive")
    if not neg_sigs:
        errors.append("No function signatures found in negative")
    
    if pos_sigs and neg_sigs:
        # Compare first function signatures
        sim = signature_similarity(pos_sigs[0], neg_sigs[0])
        if sim < 0.85:
            errors.append(f"Signature similarity {sim:.2f} < 0.85 — tests naming, not contract violation")
    
    # Property 2: Positive must have at least one conditional
    pos_conditionals = count_conditionals(positive_path)
    if pos_conditionals < 1:
        errors.append(f"CSA positive has {pos_conditionals} conditionals — too shallow, signature-only discriminator")
    
    # Property 3: Line count floor
    pos_lines = count_lines(positive_path)
    if pos_lines < 12:
        errors.append(f"CSA positive has {pos_lines} lines (minimum 12) — likely signature-dominated scoring")
    
    # Property 4: Must have branch with no rejecting path
    if not has_branch_with_no_rejecting_path(positive_path):
        errors.append("Positive must contain a branch that LOOKS like a check but doesn't reject")
    
    return errors


def find_csa_pairs(corpus_dir: Path) -> list[tuple[Path, Path]]:
    """Find all CSA pattern pairs in corpus directory."""
    pairs = []
    
    for pos_file in sorted(corpus_dir.glob("*csa*_positive.*")):
        # Determine negative file
        neg_file = pos_file.with_name(
            pos_file.name.replace("_positive", "_negative")
        )
        if neg_file.exists():
            pairs.append((pos_file, neg_file))
    
    return pairs


def main():
    parser = argparse.ArgumentParser(description="Validate CSA pattern depth")
    parser.add_argument(
        "files",
        nargs="*",
        help="Specific positive files to validate"
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Validate all CSA patterns in corpus/targets/"
    )
    parser.add_argument(
        "--corpus-dir",
        type=Path,
        default=Path("corpus/targets"),
        help="Corpus directory (default: corpus/targets/)"
    )
    args = parser.parse_args()
    
    if not args.all and not args.files:
        parser.error("Specify files or use --all")
    
    if args.all:
        pairs = find_csa_pairs(args.corpus_dir)
    else:
        pairs = []
        for f in args.files:
            pos_path = Path(f)
            neg_path = pos_path.with_name(
                pos_path.name.replace("_positive", "_negative")
            )
            if neg_path.exists():
                pairs.append((pos_path, neg_path))
            else:
                print(f"Warning: No negative file for {pos_path}")
    
    if not pairs:
        print("No CSA pairs found to validate")
        sys.exit(1)
    
    total_errors = 0
    for pos, neg in pairs:
        errors = validate_csa_pair(pos, neg)
        if errors:
            print(f"\nFAIL: {pos.name}")
            for error in errors:
                print(f"  - {error}")
            total_errors += len(errors)
        else:
            print(f"PASS: {pos.name}")
    
    if total_errors > 0:
        print(f"\n{total_errors} validation errors found")
        sys.exit(1)
    else:
        print(f"\nAll {len(pairs)} CSA pairs passed validation")
        sys.exit(0)


if __name__ == "__main__":
    main()
