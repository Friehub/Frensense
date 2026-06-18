// Vulnerable: Use of `eval` in a Claude Code or Cursor hook script is dangerous. The `eval` command re-parses its arguments, which can lead to command injection if any variable contains special characters or attacker-controlled data. Avoid `eval` entirely; use arrays, direct command invocation, or other safe alternatives.
// Pattern: eval $...ARGS
function vulnerable() {
  // TODO: implement pattern match
}
