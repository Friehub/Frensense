// Vulnerable: Detected enabling of "XMLParseNoEnt", which allows parsing of external entities and can lead to XXE if user controlled data is parsed by the library. Instead, do not enable "XMLParseNoEnt" or be sure to adequately sanitize user-controlled data when it is being parsed by this library.
// Pattern: $PARSER := parser.New(parser.XMLParseNoEnt)
function vulnerable() {
  // TODO: implement pattern match
}
