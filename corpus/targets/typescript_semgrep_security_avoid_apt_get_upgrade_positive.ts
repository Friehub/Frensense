// Vulnerable: Packages in base containers should be up-to-date, removing the need to upgrade or dist-upgrade. If a package is out of date, contact the maintainers.
// Pattern: {'pattern': 'RUN ... apt-get upgrade ...'} | {'pattern': 'RUN ... apt-get dist-upgrade ...'}
function vulnerable() {
  // TODO: implement pattern match
}
