// Fixed: Calling mb_ereg_replace with user input in the options can lead to arbitrary code execution. The eval modifier (`e`) evaluates the replacement argument as code.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
