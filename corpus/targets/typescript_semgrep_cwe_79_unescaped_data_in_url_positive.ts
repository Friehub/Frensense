// Vulnerable: Found a formatted template string passed to 'template.URL()'. 'template.URL()' does not escape contents, and this could result in XSS (cross-site scripting) and therefore confidential data being stolen. Sanitize data coming into this function or make sure that no user-controlled input is coming into the function.
// Pattern: {'pattern': 'template.URL($T + $X, ...)'} | {'pattern': 'template.URL(fmt.$P("...", ...), ...)'} | {'pattern': '$T = "..."\n...\n$T = $FXN(..., $T, ...)\n...\ntemplate.URL($T, ...)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
