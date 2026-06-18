// Vulnerable: Only use DataContractResolver if you are completely sure of what information is being serialized. Malicious types can cause unexpected behavior.
// Pattern: class $MYDCR : DataContractResolver { ... }
function vulnerable() {
  // TODO: implement pattern match
}
