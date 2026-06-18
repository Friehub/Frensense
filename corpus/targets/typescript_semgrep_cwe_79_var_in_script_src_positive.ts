// Vulnerable: Detected a template variable used as the 'src' in a script tag. Although template variables are HTML escaped, HTML escaping does not always prevent malicious URLs from being injected and could results in a cross-site scripting (XSS) vulnerability. Prefer not to dynamically generate the 'src' attribute and use static URLs instead. If you must do this, carefully check URLs against an allowlist and be sure to URL-encode the result.
// Pattern: <% ... >
function vulnerable() {
  // TODO: implement pattern match
}
