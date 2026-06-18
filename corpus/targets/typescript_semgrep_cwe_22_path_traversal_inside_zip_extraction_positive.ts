// Vulnerable: File traversal when extracting zip archive
// Pattern: reader, $ERR := zip.OpenReader($ARCHIVE)
...
for _, $FILE := range reader.File {
  ...
  path := filepath.Join($TARGET, $FILE.Name)
  ...
}
function vulnerable() {
  // TODO: implement pattern match
}
