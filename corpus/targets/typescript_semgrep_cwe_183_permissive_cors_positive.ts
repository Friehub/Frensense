// Vulnerable: https://find-sec-bugs.github.io/bugs.htm#PERMISSIVE_CORS Permissive CORS policy will allow a malicious application to communicate with the victim application in an inappropriate way, leading to spoofing, data theft, relay and other attacks.
// Pattern: {'pattern': 'HttpServletResponse $RES = ...;\n...\n$RES.addHeader("=~/access-control-allow-origin/i", "=~/^\\*|null$/i");\n'} | {'pattern': 'HttpServletResponse $RES = ...;\n...\n$RES.setHeader("=~/access-control-allow-origin/i", "=~/^\\*|null$/i");\n'} | {'pattern': 'ServerHttpResponse $RES = ...;\n...\n$RES.getHeaders().add("=~/access-control-allow-origin/i", "=~/^\\*|null$/i");\n'}
function vulnerable() {
  // TODO: implement pattern match
}
