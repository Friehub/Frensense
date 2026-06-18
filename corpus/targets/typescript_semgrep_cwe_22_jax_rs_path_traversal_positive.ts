// Vulnerable: Detected a potential path traversal. A malicious actor could control the location of this file, to include going backwards in the directory with '../'. To address this, ensure that user-controlled variables in file paths are sanitized. You may also consider using a utility method such as org.apache.commons.io.FilenameUtils.getName(...) to only retrieve the file name from the path.
// Pattern: {'pattern': '$RETURNTYPE $FUNC (..., @PathParam(...) $TYPE $VAR, ...) {\n  ...\n  new File(..., $VAR, ...);\n  ...\n}\n'} | {'pattern': '$RETURNTYPE $FUNC (..., @javax.ws.rs.PathParam(...) $TYPE $VAR, ...) {\n  ...\n  new File(..., $VAR, ...);\n  ...\n}'}
function vulnerable() {
  // TODO: implement pattern match
}
