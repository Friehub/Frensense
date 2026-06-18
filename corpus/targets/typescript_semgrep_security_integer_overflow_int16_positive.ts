// Vulnerable: Detected conversion of the result of a strconv.Atoi command to an int16. This could lead to an integer overflow, which could possibly result in unexpected behavior and even privilege escalation. Instead, use `strconv.ParseInt`.
// Pattern: $F, $ERR := strconv.Atoi($NUM)
...
int16($F)
function vulnerable() {
  // TODO: implement pattern match
}
