// Vulnerable: Use of 'ondoctype' in 'sax' library detected. By default, 'sax' won't do anything with custom DTD entity definitions. If you're implementing a custom DTD entity definition, be sure not to introduce XML External Entity (XXE) vulnerabilities, or be absolutely sure that external entities received from a trusted source while processing XML.
// Pattern: {'pattern': "require('sax');\n...\n$PARSER.ondoctype = ...;\n"} | {'pattern': "require('sax');\n...\n$PARSER.on('doctype',...);"}
function vulnerable() {
  // TODO: implement pattern match
}
