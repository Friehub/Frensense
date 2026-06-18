// Fixed: Found a formatted template string passed to 'template.URL()'. 'template.URL()' does not escape contents, and this could result in XSS (cross-site scripting) and therefore confidential data being stolen. Sanitize data coming into this function or make sure that no user-controlled input is coming into the function.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
