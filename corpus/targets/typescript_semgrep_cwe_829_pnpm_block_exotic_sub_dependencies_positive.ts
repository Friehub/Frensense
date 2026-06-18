// Vulnerable: Missing or incorrect blockExoticSubdeps. Set `blockExoticSubdeps: true` to transitive dependencies from being installed from untrusted sources. Added in: v10.26.0 Reference: https://pnpm.io/settings#blockexoticsubdeps
// Pattern: {'patterns': [{'pattern-regex': '(?ms)(?:\\A|^---$\\n)(?:(?!^blockExoticSubdeps\\s*:)(?!^---$)[\\s\\S])*?(?P<TARGET>^(?:packages|catalog)\\s*:)(?:(?!^blockExoticSubdeps\\s*:)(?!^---$)[\\s\\S])*?(?=^---$|\\z)'}, {'focus-metavariable': '$TARGET'}]} | {'patterns': [{'pattern': 'blockExoticSubdeps: $VAL\n'}, {'metavariable-regex': {'metavariable': '$VAL', 'regex': '^(?!true$).+'}}, {'focus-metavariable': '$VAL'}]} | {'patterns': [{'pattern-regex': '(?m)^\\s*blockExoticSubdeps\\s*:\\s*$'}]}
function vulnerable() {
  // TODO: implement pattern match
}
