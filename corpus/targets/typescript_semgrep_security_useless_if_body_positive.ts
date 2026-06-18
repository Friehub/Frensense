// Vulnerable: Detected identical statements in the if body and the else body of an if-statement. This will lead to the same code being executed no matter what the if-expression evaluates to. Instead, remove the if statement.
// Pattern: if ($X) {
    $S
} else {
    $S
}
function vulnerable() {
  // TODO: implement pattern match
}
