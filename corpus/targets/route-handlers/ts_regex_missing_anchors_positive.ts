// [frensense]
// observation: Regex validation pattern missing ^ and $ anchors, allowing partial matches against invalid input.
// impact: A pattern like /\\d{5}/ matches 'abc12345xyz' as valid, even though the full value should be exactly 5 digits. Attackers can inject malicious content as a prefix or suffix.
// improvement: Always anchor regex patterns with ^ at the start and $ at the end for full-string validation.
// cwe: CWE-1333
// cvss: 7.5
// owasp: A06:2021
// severity: High

function isValidZipCode(zip: string): boolean {
  // VULNERABLE: no anchors — matches substring
  return /\d{5}/.test(zip);
}

function isValidUsername(username: string): boolean {
  // VULNERABLE: no end anchor — allows trailing content
  return /^[a-z0-9_]{3,16}/.test(username);
}

app.post('/api/validate-input', (req, res) => {
  const { zipCode, username } = req.body;
  if (isValidZipCode(zipCode) && isValidUsername(username)) {
    return res.json({ valid: true });
  }
  res.json({ valid: false });
});
