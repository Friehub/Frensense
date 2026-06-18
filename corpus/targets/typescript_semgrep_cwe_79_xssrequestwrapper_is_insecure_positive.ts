// Vulnerable: It looks like you're using an implementation of XSSRequestWrapper from dzone. (https://www.javacodegeeks.com/2012/07/anti-cross-site-scripting-xss-filter.html) The XSS filtering in this code is not secure and can be bypassed by malicious actors. It is recommended to use a stack that automatically escapes in your view or templates instead of filtering yourself.
// Pattern: {'pattern': 'class XSSRequestWrapper extends HttpServletRequestWrapper {\n  ...\n}\n'} | {'pattern': '$P = $X.compile("</script>", $X.CASE_INSENSITIVE);\n$V = $P.matcher(...).replaceAll("");'}
function vulnerable() {
  // TODO: implement pattern match
}
