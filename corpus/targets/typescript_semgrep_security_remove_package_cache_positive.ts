// Vulnerable: The package cache was not deleted after running 'apt-get update', which increases the size of the image. Remove the package cache by appending '&& apt-get clean' at the end of apt-get command chain.
// Pattern: RUN ... apt-get update ...
function vulnerable() {
  // TODO: implement pattern match
}
