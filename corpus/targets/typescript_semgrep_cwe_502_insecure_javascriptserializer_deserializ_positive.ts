// Vulnerable: The SimpleTypeResolver class is insecure and should not be used. Using SimpleTypeResolver to deserialize JSON could allow the remote client to execute malicious code within the app and take control of the web server.
// Pattern: new JavaScriptSerializer((SimpleTypeResolver $RESOLVER))
function vulnerable() {
  // TODO: implement pattern match
}
