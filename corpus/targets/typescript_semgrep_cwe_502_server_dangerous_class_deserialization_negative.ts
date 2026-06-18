// Fixed: Using a non-primitive class with Java RMI may be an insecure deserialization vulnerability. Depending on the underlying implementation. This object could be manipulated by a malicious actor allowing them to execute code on your system. Instead, use an integer ID to look up your object, or consider alternative serialization schemes such as JSON.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
