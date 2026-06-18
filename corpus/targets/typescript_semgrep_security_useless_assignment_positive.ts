// Vulnerable: `$X` is assigned twice; the first assignment is useless
// Pattern: $X = $Y;
$X = $Z;
function vulnerable() {
  // TODO: implement pattern match
}
