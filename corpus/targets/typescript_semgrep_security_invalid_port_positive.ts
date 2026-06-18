// Vulnerable: Detected an invalid port number. Valid ports are 0 through 65535.
// Pattern: {'patterns': [{'pattern': 'EXPOSE $PORT'}, {'metavariable-comparison': {'metavariable': '$PORT', 'comparison': 'int($PORT) > 65535'}}]}
function vulnerable() {
  // TODO: implement pattern match
}
