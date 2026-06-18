// Vulnerable: MemoryMarshal.CreateSpan and MemoryMarshal.CreateReadOnlySpan should be used with caution, as the length argument is not checked.
// Pattern: {'pattern': 'MemoryMarshal.CreateSpan(...)'} | {'pattern': 'MemoryMarshal.CreateReadOnlySpan(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
