// Vulnerable: 'open_in' behaves differently on Windows and on Unix-like systems with respect to line endings. To get the same behavior everywhere, use 'open_in_bin' or 'open_in_gen [Open_binary]'. If you really want CRLF-to-LF translations to take place when running on Windows, use 'open_in_gen [Open_text]'.
// Pattern: open_in
function vulnerable() {
  // TODO: implement pattern match
}
