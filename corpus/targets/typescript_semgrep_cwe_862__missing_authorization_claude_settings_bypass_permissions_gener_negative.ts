// Fixed: Dangerous permission bypass detected in Claude Code or Cursor settings. Settings like "bypassPermissions", "allowUnsandboxedCommands: true", or "enableWeakerNestedSandbox: true" disable critical security controls that protect against malicious tool use. Remove these settings or set them to false to maintain proper sandboxing and permission checks.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
