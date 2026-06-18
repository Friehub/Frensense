// Vulnerable: Checks for requests to http (unencrypted) sites using gorequest, a popular HTTP client library. This is dangerous because it could result in plaintext PII being passed around the network.
// Pattern: {'patterns': [{'pattern-inside': '$REQ = gorequest.New()\n...\n$RES = ...\n'}, {'pattern': '$REQ.$FUNC("=~/[hH][tT][tT][pP]://.*/")\n'}, {'metavariable-regex': {'metavariable': '$FUNC', 'regex': '(Get|Post|Delete|Head|Put|Patch)'}}]} | {'patterns': [{'pattern': 'gorequest.New().$FUNC("=~/[hH][tT][tT][pP]://.*/")'}, {'metavariable-regex': {'metavariable': '$FUNC', 'regex': '(Get|Post|Delete|Head|Put|Patch)'}}]}
function vulnerable() {
  // TODO: implement pattern match
}
