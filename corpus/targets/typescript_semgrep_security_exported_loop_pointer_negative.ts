// Fixed: `$VALUE` is a loop pointer that may be exported from the loop. This pointer is shared between loop iterations, so the exported reference will always point to the last loop value, which is likely unintentional. To fix, copy the pointer to a new pointer within the loop.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
