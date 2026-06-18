// Fixed: The package cache was not deleted after running 'apt-get update', which increases the size of the image. Remove the package cache by appending '&& apt-get clean' at the end of apt-get command chain.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
