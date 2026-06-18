// Vulnerable: The host for this proxy URL is dynamically determined. This can be dangerous if the host can be injected by an attacker because it may forcibly alter destination of the proxy. Consider hardcoding acceptable destinations and retrieving them with 'map' or something similar.
// Pattern: {'pattern': 'proxy_pass $SCHEME://$$HOST ...;'} | {'pattern': 'proxy_pass $$SCHEME://$$HOST ...;'}
function vulnerable() {
  // TODO: implement pattern match
}
