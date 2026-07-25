// [frensense]
// observation: Regex pattern with nested quantifiers (e.g., (a+)+) or overlapping alternations causes catastrophic backtracking on user-controlled input.
// impact: An attacker can craft a string (e.g., 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac') that causes the regex engine to explore exponential permutations, freezing the CPU for seconds or minutes. This is a ReDoS (Regular Expression Denial of Service) attack.
// improvement: Avoid nested quantifiers. Use atomic groups, possessive quantifiers, or a non-backtracking regex engine. Set a timeout on regex execution.
// cwe: CWE-1333
// cvss: 7.5
// owasp: A06:2021
// severity: High

app.post('/api/validate', (req, res) => {
  const { input } = req.body;

  // VULNERABLE: nested quantifier — catastrophic backtracking
  const EMAIL_REGEX = /^([a-zA-Z]+)+@example\.com$/;
  if (EMAIL_REGEX.test(input)) {
    return res.json({ valid: true });
  }
  res.json({ valid: false });
});

app.post('/api/search', (req, res) => {
  // VULNERABLE: (a+)+b pattern
  const searchPattern = new RegExp(`^${req.body.pattern}+$`);
  if (searchPattern.test(req.body.input)) {
    // ...
  }
});
