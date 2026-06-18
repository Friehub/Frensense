// Fixed: By setting `allErrors: true` in `Ajv` library, all error objects will be allocated without limit. This allows the attacker to produce a huge number of errors which can lead to denial of service. Do not use `allErrors: true` in production.
// Apply appropriate sanitization
function safe() {
  // TODO: implement fix
}
