// Vulnerable: The package lists were not deleted after running 'apt-get update', which increases the size of the image. Remove the package lists by appending '&& rm -rf /var/lib/apt/lists/*' at the end of apt-get command chain.
// Pattern: RUN apt-get update ...
function vulnerable() {
  // TODO: implement pattern match
}
