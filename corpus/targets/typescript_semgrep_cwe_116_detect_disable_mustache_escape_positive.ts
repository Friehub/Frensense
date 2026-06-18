// Vulnerable: Markup escaping disabled. This can be used with some template engines to escape disabling of HTML entities, which can lead to XSS attacks.
// Pattern: $OBJ.escapeMarkup = false
function vulnerable() {
  // TODO: implement pattern match
}
