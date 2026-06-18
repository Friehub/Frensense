// Vulnerable: The protocol scheme for this proxy is dynamically determined. This can be dangerous if the scheme can be injected by an attacker because it may forcibly alter the connection scheme. Consider hardcoding a scheme for this proxy.
// Pattern: proxy_pass $$SCHEME:// ...;
function vulnerable() {
  // TODO: implement pattern match
}
