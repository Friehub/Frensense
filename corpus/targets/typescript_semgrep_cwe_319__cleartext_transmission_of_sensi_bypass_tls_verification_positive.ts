// Vulnerable: Checks for setting the environment variable NODE_TLS_REJECT_UNAUTHORIZED to 0, which disables TLS verification. This should only be used for debugging purposes. Setting the option rejectUnauthorized to false bypasses verification against the list of trusted CAs, which also leads to insecure transport. These options lead to vulnerability to MTM attacks, and should not be used.
// Pattern: {'pattern': 'process.env["NODE_TLS_REJECT_UNAUTHORIZED"] = 0;\n'} | {'pattern': '{rejectUnauthorized:false}\n'}
function vulnerable() {
  // TODO: implement pattern match
}
