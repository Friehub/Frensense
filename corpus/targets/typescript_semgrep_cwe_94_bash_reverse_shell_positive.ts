// Vulnerable: Semgrep found a bash reverse shell
// Pattern: {'pattern': 'sh -i >& /dev/udp/.../... 0>&1\n'} | {'pattern': '<...>/dev/tcp/.../...; sh <&... >&... 2>&\n'} | {'pattern': '<...>/dev/tcp/.../...; cat <&... | while read line; do $line 2>&... >&...;done\n'}
function vulnerable() {
  // TODO: implement pattern match
}
