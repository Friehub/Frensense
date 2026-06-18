// Vulnerable: Detected use of a Java socket that is not encrypted. As a result, the traffic could be read by an attacker intercepting the network traffic. Use an SSLSocket created by 'SSLSocketFactory' or 'SSLServerSocketFactory' instead.
// Pattern: {'pattern': 'new ServerSocket(...)'} | {'pattern': 'new Socket(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
