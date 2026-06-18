// Vulnerable: Detected a relative WORKDIR. Use absolute paths. This prevents issues based on assumptions about the WORKDIR of previous containers.
// Pattern: {'patterns': [{'pattern': 'WORKDIR $VALUE'}, {'metavariable-pattern': {'metavariable': '$VALUE', 'patterns': [{'pattern-not-regex': '(\\/.*)'}]}}]} | {'patterns': [{'pattern': 'ENV $VAR=$VALUE ... $CMD ${$VAR}'}, {'metavariable-pattern': {'metavariable': '$VALUE', 'patterns': [{'pattern-not-regex': '(\\/.*)'}]}}, {'metavariable-pattern': {'metavariable': '$CMD', 'pattern': 'WORKDIR'}}, {'focus-metavariable': '$CMD'}]}
function vulnerable() {
  // TODO: implement pattern match
}
