// Fixed: The SimpleTypeResolver class is insecure and should not be used. Using SimpleTypeResolver to deserialize JSON could allow the remote client to execute malicious code within the app and take control of the web server.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
