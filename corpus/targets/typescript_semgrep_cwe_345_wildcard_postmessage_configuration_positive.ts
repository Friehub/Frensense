// Vulnerable: The target origin of the window.postMessage() API is set to "*". This could allow for information disclosure due to the possibility of any origin allowed to receive the message.
// Pattern: $OBJECT.postMessage(...,'*',...)
function vulnerable() {
  // TODO: implement pattern match
}
