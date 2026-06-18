// Vulnerable: Prefer JSON notation when using CMD or ENTRYPOINT. This allows signals to be passed from the OS.
// Pattern: {'patterns': [{'pattern': 'CMD $WORD ...'}, {'pattern-not-inside': 'CMD [...]'}]} | {'patterns': [{'pattern': 'ENTRYPOINT $WORD ...'}, {'pattern-not-inside': 'ENTRYPOINT [...]'}]}
function vulnerable() {
  // TODO: implement pattern match
}
