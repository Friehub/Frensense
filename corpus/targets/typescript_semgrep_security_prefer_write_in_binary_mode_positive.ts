// Vulnerable: 'open_out' behaves differently on Windows and on Unix-like systems with respect to line endings. To get the same behavior everywhere, use 'open_out_bin' or 'open_out_gen [Open_binary]'. If you really want LF-to-CRLF translations to take place when running on Windows, use 'open_out_gen [Open_text]'.
// Pattern: open_out
function vulnerable() {
  // TODO: implement pattern match
}
