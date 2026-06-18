// Vulnerable: This 'apt-get install' is missing '--no-install-recommends'. This prevents unnecessary packages from being installed, thereby reducing image size. Add '--no-install-recommends'.
// Pattern: RUN apt-get install ...
function vulnerable() {
  // TODO: implement pattern match
}
