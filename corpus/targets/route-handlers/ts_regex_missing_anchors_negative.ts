// SAFE: anchored patterns
function isValidZipCode(zip: string): boolean {
  return /^\d{5}$/.test(zip);
}

function isValidUsername(username: string): boolean {
  return /^[a-z0-9_]{3,16}$/.test(username);
}

app.post('/api/validate-input', (req, res) => {
  const { zipCode, username } = req.body;
  if (isValidZipCode(zipCode) && isValidUsername(username)) {
    return res.json({ valid: true });
  }
  res.json({ valid: false });
});
