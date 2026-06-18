// Fixed: The package lists were not deleted after running 'apt-get update', which increases the size of the image. Remove the package lists by appending '&& rm -rf /var/lib/apt/lists/*' at the end of apt-get command chain.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
