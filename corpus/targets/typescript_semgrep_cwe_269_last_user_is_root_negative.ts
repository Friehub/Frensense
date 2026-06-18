// Fixed: The last user in the container is 'root'. This is a security hazard because if an attacker gains control of the container they will have root access. Switch back to another user after running commands as 'root'.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
