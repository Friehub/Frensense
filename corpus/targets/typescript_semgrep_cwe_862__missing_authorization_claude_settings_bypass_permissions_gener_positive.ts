// Vulnerable: Dangerous permission bypass detected in Claude Code or Cursor settings. Settings like "bypassPermissions", "allowUnsandboxedCommands: true", or "enableWeakerNestedSandbox: true" disable critical security controls that protect against malicious tool use. Remove these settings or set them to false to maintain proper sandboxing and permission checks.
// Pattern: {'pattern-regex': '"bypassPermissions"'} | {'pattern-regex': '"allowUnsandboxedCommands"\\s*:\\s*true'} | {'pattern-regex': '"enableWeakerNestedSandbox"\\s*:\\s*true'}
function vulnerable() {
  // TODO: implement pattern match
}
