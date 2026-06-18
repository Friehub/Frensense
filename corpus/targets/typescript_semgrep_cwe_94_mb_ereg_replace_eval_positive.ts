// Vulnerable: Calling mb_ereg_replace with user input in the options can lead to arbitrary code execution. The eval modifier (`e`) evaluates the replacement argument as code.
// Pattern: mb_ereg_replace($PATTERN, $REPL, $STR, $OPTIONS);
function vulnerable() {
  // TODO: implement pattern match
}
