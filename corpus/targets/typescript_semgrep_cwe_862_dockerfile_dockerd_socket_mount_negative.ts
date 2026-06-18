// Fixed: The Dockerfile(image) mounts docker.sock to the container which may allow an attacker already inside of the container to escape container and execute arbitrary commands on the host machine.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
