// Vulnerable: Finding triggers whenever there is a strcat or strncat used. This is an issue because strcat or strncat can lead to buffer overflow vulns. Fix this by using strcat_s instead.
// Pattern: {'pattern': 'strcat(...)'} | {'pattern': 'strncat(...)'}
function vulnerable() {
  // TODO: implement pattern match
}
