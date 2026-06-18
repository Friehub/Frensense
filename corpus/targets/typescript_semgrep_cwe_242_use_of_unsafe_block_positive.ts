// Vulnerable: Using the unsafe package in Go gives you low-level memory management and many of the strengths of the C language, but also steps around the type safety of Go and can lead to buffer overflows and possible arbitrary code execution by an attacker. Only use this package if you absolutely know what you're doing.
// Pattern: unsafe.$FUNC(...)
function vulnerable() {
  // TODO: implement pattern match
}
