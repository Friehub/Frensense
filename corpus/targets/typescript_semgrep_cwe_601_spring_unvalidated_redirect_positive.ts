// Vulnerable: Application redirects a user to a destination URL specified by a user supplied parameter that is not validated.
// Pattern: {'pattern': '$X $METHOD(...,String $URL,...) {\n  return "redirect:" + $URL;\n}\n'} | {'pattern': '$X $METHOD(...,String $URL,...) {\n  ...\n  String $REDIR = "redirect:" + $URL;\n  ...\n  return $REDIR;\n  ...\n}\n'} | {'pattern': '$X $METHOD(...,String $URL,...) {\n  ...\n  new ModelAndView("redirect:" + $URL);\n  ...\n}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
