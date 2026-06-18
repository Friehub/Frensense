// Vulnerable: Application redirects to a destination URL specified by a user-supplied parameter that is not validated. This could direct users to malicious locations. Consider using an allowlist to validate URLs.
// Pattern: {'pattern': '$X $METHOD(...,HttpServletResponse $RES,...,String $URL,...) {\n  ...\n  $RES.sendRedirect($URL);\n  ...\n}\n'} | {'pattern': '$X $METHOD(...,String $URL,...,HttpServletResponse $RES,...) {\n  ...\n  $RES.sendRedirect($URL);\n  ...\n}\n'} | {'pattern': '$X $METHOD(...,HttpServletRequest $REQ,...,HttpServletResponse $RES,...) {\n  ...\n  String $URL = $REQ.getParameter(...);\n  ...\n  $RES.sendRedirect($URL);\n  ...\n}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
