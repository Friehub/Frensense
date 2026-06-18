// Vulnerable: These hooks allow the developer to handle the custom AJAX endpoints."wp_ajax_$action" hook get fires for any authenticated user and "wp_ajax_nopriv_$action" hook get fires for non-authenticated users.
// Pattern: add_action($HOOK,...)
function vulnerable() {
  // TODO: implement pattern match
}
