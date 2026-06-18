// Vulnerable: URL rewriting has significant security risks. Since session ID appears in the URL, it may be easily seen by third parties.
// Pattern: {'pattern': '$X $METHOD(...,HttpServletResponse $RES,...) {\n  ...\n  $RES.encodeURL(...);\n  ...\n}\n'} | {'pattern': '$X $METHOD(...,HttpServletResponse $RES,...) {\n  ...\n  $RES.encodeUrl(...);\n  ...\n}\n'} | {'pattern': '$X $METHOD(...,HttpServletResponse $RES,...) {\n  ...\n  $RES.encodeRedirectURL(...);\n  ...\n}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
