// Vulnerable: Detected a template variable used in a script tag. Although template variables are HTML escaped, HTML escaping does not always prevent cross-site scripting (XSS) attacks when used directly in JavaScript. If you need this data on the rendered page, consider placing it in the HTML portion (outside of a script tag). Alternatively, use a JavaScript-specific encoder, such as the one available in OWASP ESAPI.
// Pattern: {'pattern-regex': 'script\\s*=[A-Za-z0-9]+'} | {'pattern-regex': 'script\\s*=.*["\']\\s*\\+.*'} | {'pattern-regex': 'script\\s*=[^\'"]+\\+.*'}
function vulnerable() {
  // TODO: implement pattern match
}
