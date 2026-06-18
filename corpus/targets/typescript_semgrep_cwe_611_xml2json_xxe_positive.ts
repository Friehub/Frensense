// Vulnerable: If unverified user data can reach the XML Parser it can result in XML External or Internal Entity (XXE) Processing vulnerabilities
// Pattern: var $XML = require('xml2json');
...
$XML.toJson(...);
function vulnerable() {
  // TODO: implement pattern match
}
