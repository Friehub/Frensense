// Vulnerable: Found a formatted template string passed to 'template. HTMLAttr()'. 'template.HTMLAttr()' does not escape contents. Be absolutely sure there is no user-controlled data in this template or validate and sanitize the data before passing it into the template.
// Pattern: {'pattern': 'template.HTMLAttr($T + $X, ...)'} | {'pattern': 'template.HTMLAttr(fmt.$P("...", ...), ...)'} | {'pattern': '$T = "..."\n...\n$T = $FXN(..., $T, ...)\n...\ntemplate.HTMLAttr($T, ...)\n'}
function vulnerable() {
  // TODO: implement pattern match
}
