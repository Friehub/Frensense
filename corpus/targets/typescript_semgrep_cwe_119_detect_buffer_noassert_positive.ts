// Vulnerable: Detected usage of noassert in Buffer API, which allows the offset the be beyond the end of the buffer. This could result in writing or reading beyond the end of the buffer.
// Pattern: $OBJ.$API(..., true)
function vulnerable() {
  // TODO: implement pattern match
}
