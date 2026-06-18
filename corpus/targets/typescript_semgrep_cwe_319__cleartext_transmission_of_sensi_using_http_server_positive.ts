// Vulnerable: Checks for any usage of http servers instead of https servers. Encourages the usage of https protocol instead of http, which does not have TLS and is therefore unencrypted. Using http can lead to man-in-the-middle attacks in which the attacker is able to read sensitive information.
// Pattern: $HTTP
function vulnerable() {
  // TODO: implement pattern match
}
