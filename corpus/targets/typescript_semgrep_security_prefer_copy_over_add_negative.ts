// Fixed: The ADD command will accept and include files from a URL and automatically extract archives. This potentially exposes the container to a man-in-the-middle attack or other attacks if a malicious actor can tamper with the source archive. Since ADD can have this and other unexpected side effects, the use of the more explicit COPY command is preferred.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
