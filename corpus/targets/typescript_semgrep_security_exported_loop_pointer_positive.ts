// Vulnerable: `$VALUE` is a loop pointer that may be exported from the loop. This pointer is shared between loop iterations, so the exported reference will always point to the last loop value, which is likely unintentional. To fix, copy the pointer to a new pointer within the loop.
// Pattern: {'pattern': 'for _, $VALUE := range $SOURCE {\n  <... &($VALUE) ...>\n}\n'} | {'pattern': 'for _, $VALUE := range $SOURCE {\n  <... func() { <... &$VALUE ...> } ...>\n}\n'} | {'pattern': 'for _, $VALUE := range $SOURCE {\n  <... $ANYTHING(..., <... &$VALUE ...>, ...) ...>\n}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
