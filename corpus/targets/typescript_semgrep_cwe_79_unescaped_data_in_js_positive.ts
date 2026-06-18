// Vulnerable: Found a formatted template string passed to 'template.JS()'. 'template.JS()' does not escape contents. Be absolutely sure there is no user-controlled data in this template.
// Pattern: {'pattern': 'template.JS($T + $X, ...)'} | {'pattern': 'template.JS(fmt.$P("...", ...), ...)'} | {'pattern': '$T = "..."\n...\n$T = $FXN(..., $T, ...)\n...\ntemplate.JS($T, ...)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
