// Vulnerable: In $METHOD, $X is used to construct a SQL query via string concatenation.
// Pattern: {'pattern': '$RETURN $METHOD(...,String $X,...){\n  ...\n  Session $SESSION = ...;\n  ...\n  String $QUERY = ... + $X + ...;\n  ...\n  PreparedStatement $PS = $SESSION.connection().prepareStatement($QUERY);\n  ...\n  ResultSet $RESULT = $PS.executeQuery();\n  ...\n}\n'} | {'pattern': '$RETURN $METHOD(...,String $X,...){\n  ...\n  String $QUERY = ... + $X + ...;\n  ...\n  Session $SESSION = ...;\n  ...\n  PreparedStatement $PS = $SESSION.connection().prepareStatement($QUERY);\n  ...\n  ResultSet $RESULT = $PS.executeQuery();\n  ...\n}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
