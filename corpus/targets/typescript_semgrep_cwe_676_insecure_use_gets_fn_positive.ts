// Vulnerable: Avoid 'gets()'. This function does not consider buffer boundaries and can lead to buffer overflows. Use 'fgets()' or 'gets_s()' instead.
// Pattern: gets(...)
function vulnerable() {
  // TODO: implement pattern match
}
