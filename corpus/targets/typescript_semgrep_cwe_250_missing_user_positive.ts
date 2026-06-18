// Vulnerable: By not specifying a USER, a program in the container may run as 'root'. This is a security hazard. If an attacker can control a process running as root, they may have control over the container. Ensure that the last USER in a Dockerfile is a USER other than 'root'.
// Pattern: CMD $...VARS
function vulnerable() {
  // TODO: implement pattern match
}
