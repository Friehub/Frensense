// Vulnerable: $sceDelegateProvider allowlisting can introduce security issues if wildcards are used.
// Pattern: {'pattern': "$sceDelegateProvider.resourceUrlWhitelist([...,'**',...]);\n"} | {'patterns': [{'pattern': '$sceDelegateProvider.resourceUrlWhitelist([...,$DOM,...]);\n'}, {'metavariable-regex': {'metavariable': '$DOM', 'regex': "^'.*\\*\\*.+'$"}}]}
function vulnerable() {
  // TODO: implement pattern match
}
