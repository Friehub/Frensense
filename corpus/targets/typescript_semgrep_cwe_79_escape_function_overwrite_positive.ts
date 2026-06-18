// Vulnerable: The Mustache escape function is being overwritten. This could bypass HTML escaping safety measures built into the rendering engine, exposing your application to cross-site scripting (XSS) vulnerabilities. If you need unescaped HTML, use the triple brace operator in your template: '{{{ ... }}}'.
// Pattern: {'pattern': 'Mustache.escape = ...'} | {'patterns': [{'pattern-inside': '$MUSTACHE = require("mustache");\n...\n'}, {'pattern': '$MUSTACHE.escape = ...'}]}
function vulnerable() {
  // TODO: implement pattern match
}
