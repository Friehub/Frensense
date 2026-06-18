// Vulnerable: File creation in shared tmp directory without using `io.CreateTemp`.
// Pattern: {'pattern': 'ioutil.WriteFile("=~//tmp/.*$/", ...)'} | {'pattern': 'os.Create("=~//tmp/.*$/", ...)'} | {'pattern': 'os.WriteFile("=~//tmp/.*$/", ...)'}
function vulnerable() {
  // TODO: implement pattern match
}
