// Vulnerable: Finding triggers whenever there is a strcpy or strncpy used. This is an issue because strcpy does not affirm the size of the destination array and strncpy will not automatically NULL-terminate strings. This can lead to buffer overflows, which can cause program crashes and potentially let an attacker inject code in the program. Fix this by using strcpy_s instead (although note that strcpy_s is an optional part of the C11 standard, and so may not be available).
// Pattern: {'pattern': 'strcpy(...)'} | {'pattern': 'strncpy(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
