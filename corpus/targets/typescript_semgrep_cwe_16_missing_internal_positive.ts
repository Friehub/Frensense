// Vulnerable: This location block contains a 'proxy_pass' directive but does not contain the 'internal' directive. The 'internal' directive restricts access to this location to internal requests. Without 'internal', an attacker could use your server for server-side request forgeries (SSRF). Include the 'internal' directive in this block to limit exposure.
// Pattern: proxy_pass $...URL;
function vulnerable() {
  // TODO: implement pattern match
}
