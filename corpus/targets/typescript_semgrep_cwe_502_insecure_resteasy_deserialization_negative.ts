// Fixed: When a Restful webservice endpoint is configured to use wildcard mediaType {*/*} as a value for the @Consumes annotation, an attacker could abuse the SerializableProvider by sending a HTTP Request with a Content-Type of application/x-java-serialized-object. The body of that request would be processed by the SerializationProvider and could contain a malicious payload, which may lead to arbitrary code execution when calling the $Y.getObject method.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
