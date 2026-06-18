// Vulnerable: Marshaling is currently not type-safe and can lead to insecure behaviour when untrusted data is marshalled. Marshalling can lead to out-of-bound reads as well.
// Pattern: {'pattern': 'input_value'} | {'pattern': 'Marshal.from_channel'} | {'pattern': 'Marshal.from_bytes'}
function vulnerable() {
  // TODO: implement pattern match
}
