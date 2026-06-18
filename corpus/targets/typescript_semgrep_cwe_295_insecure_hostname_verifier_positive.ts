// Vulnerable: Insecure HostnameVerifier implementation detected. This will accept any SSL certificate with any hostname, which creates the possibility for man-in-the-middle attacks.
// Pattern: {'pattern': 'class $CLASS implements HostnameVerifier {\n  ...\n  public boolean verify(...) { return true; }\n}\n'} | {'pattern': 'new HostnameVerifier(...){\n  public boolean verify(...) {\n    return true;\n  }\n}'} | {'pattern': 'import org.apache.http.conn.ssl.NoopHostnameVerifier;'}
function vulnerable() {
  // TODO: implement pattern match
}
