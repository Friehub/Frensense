// Vulnerable: 'input_line' leaves a '\r' (CR) character when reading lines from a Windows text file, whose lines end in "\r\n" (CRLF). This is a problem for any Windows file that is being read either on a Unix-like platform or on Windows in binary mode. If the code already takes care of removing any trailing '\r' after reading the line, add a '(* nosemgrep *)' comment to disable this warning.
// Pattern: input_line
function vulnerable() {
  // TODO: implement pattern match
}
