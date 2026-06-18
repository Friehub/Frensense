// Vulnerable: By setting `allErrors: true` in `Ajv` library, all error objects will be allocated without limit. This allows the attacker to produce a huge number of errors which can lead to denial of service. Do not use `allErrors: true` in production.
// Pattern: {'pattern': 'new Ajv({...,allErrors: true,...},...)\n'} | {'patterns': [{'pattern': 'new Ajv($SETTINGS,...)\n'}, {'pattern-inside': '$SETTINGS = {...,allErrors: true,...}\n...\n'}]}
function vulnerable() {
  // TODO: implement pattern match
}
