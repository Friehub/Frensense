// Vulnerable: Multiple ENTRYPOINT instructions were found. Only the last one will take effect.
// Pattern: ENTRYPOINT ...
...
$ENTRYPOINT_INSTR
function vulnerable() {
  // TODO: implement pattern match
}
