// Vulnerable: Checks for cases where java applications are allowing unsafe renegotiation. This leaves the application vulnerable to a man-in-the-middle attack where chosen plain text is injected as prefix to a TLS connection.
// Pattern: java.lang.System.setProperty("sun.security.ssl.allowUnsafeRenegotiation", true);
function vulnerable() {
  // TODO: implement pattern match
}
