// Vulnerable: `$STR.replace` method will only replace the first occurrence when used with a string argument ($CHAR). If this method is used for escaping of dangerous data then there is a possibility for a bypass. Try to use sanitization library instead or use a Regex with a global flag.
// Pattern: $STR.replace(($CHAR: string), ...)
function vulnerable() {
  // TODO: implement pattern match
}
