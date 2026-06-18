// Vulnerable: This function can be used to redirect to user supplied URLs. If user input is not sanitised or validated, this could lead to Open Redirect vulnerabilities. Use "wp_safe_redirect()" to prevent this kind of attack.
// Pattern: wp_redirect(...)
function vulnerable() {
  // TODO: implement pattern match
}
