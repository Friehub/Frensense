// Vulnerable: Passing false or 0 as the third argument to this function will not cause the script to die, making the check useless.
// Pattern: check_ajax_referer(...,...,false)
function vulnerable() {
  // TODO: implement pattern match
}
