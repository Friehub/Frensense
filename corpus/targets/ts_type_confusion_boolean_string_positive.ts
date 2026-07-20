// [frensense]
// observation: String 'false' treated as truthy when evaluating boolean-like string values.
// impact: Boolean('false') returns true because 'false' is a non-empty string. This causes feature flags, admin checks, or permission gates to be bypassed. Setting 'isAdmin=false' in query params actually grants admin access.
// improvement: Use explicit comparison to 'true'/'false' strings, or use JSON.parse() for boolean conversion.

function isEnabled(flag: string): boolean {
  // VULNERABLE: any non-empty string is truthy
  return !!flag;
}

app.get('/api/feature', (req, res) => {
  // VULNERABLE: ?admin=false → admin=true
  const isAdmin = !!req.query.admin;
  if (isAdmin) {
    return res.json({ secretData: 'classified' });
  }
  res.json({ publicData: 'hello' });
});

function parseConfigValue(val: string): boolean {
  // VULNERABLE: only checks for exact 'false' string
  return val !== 'false';
}
