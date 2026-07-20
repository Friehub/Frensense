// SAFE: avoid nested quantifiers, use simple alternation
app.post('/api/validate', (req, res) => {
  const { input } = req.body;

  // SAFE: no nested quantifiers
  const EMAIL_REGEX = /^[a-zA-Z]+@example\.com$/;
  if (EMAIL_REGEX.test(input)) {
    return res.json({ valid: true });
  }
  res.json({ valid: false });
});

app.post('/api/search', (req, res) => {
  // SAFE: escape user input and avoid nested quantifiers
  const escaped = req.body.pattern.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const searchPattern = new RegExp(`^${escaped}`);
  if (searchPattern.test(req.body.input)) {
    // ...
  }
});
