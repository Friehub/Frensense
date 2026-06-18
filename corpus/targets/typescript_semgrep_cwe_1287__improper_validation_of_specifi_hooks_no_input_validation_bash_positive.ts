// Vulnerable: Piping untrusted input directly to eval, bash, or sh is dangerous in Claude Code and Cursor hooks. Validate and sanitize input before executing it.
// Pattern: {'pattern': 'eval $...ARGS'} | {'pattern': 'echo $...ARGS | bash'} | {'pattern': 'echo $...ARGS | sh'}
function vulnerable() {
  // TODO: implement pattern match
}
