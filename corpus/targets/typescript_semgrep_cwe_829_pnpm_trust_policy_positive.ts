// Vulnerable: Missing or incorrect trustPolicy. Set `trustPolicy: no-downgrade` to prevent malicious package updates from downgrading security settings. Added in: v10.21.0 Reference: https://pnpm.io/settings#trustpolicy
// Pattern: {'patterns': [{'pattern-regex': '(?ms)(?:\\A|^---$\\n)(?:(?!^trustPolicy\\s*:)(?!^---$)[\\s\\S])*?(?P<TARGET>^(?:packages|catalog)\\s*:)(?:(?!^trustPolicy\\s*:)(?!^---$)[\\s\\S])*?(?=^---$|\\z)'}, {'focus-metavariable': '$TARGET'}]} | {'patterns': [{'pattern': 'trustPolicy: $VAL\n'}, {'metavariable-regex': {'metavariable': '$VAL', 'regex': '^(?!no-downgrade$).+'}}, {'focus-metavariable': '$VAL'}]} | {'patterns': [{'pattern-regex': '(?m)^\\s*trustPolicy\\s*:\\s*$'}]}
function vulnerable() {
  // TODO: implement pattern match
}
