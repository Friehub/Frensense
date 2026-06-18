// Vulnerable: The last user in the container is 'root'. This is a security hazard because if an attacker gains control of the container they will have root access. Switch back to another user after running commands as 'root'.
// Pattern: USER root
function vulnerable() {
  // TODO: implement pattern match
}
