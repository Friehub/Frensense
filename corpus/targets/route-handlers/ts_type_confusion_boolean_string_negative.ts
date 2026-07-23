// SAFE: explicit string comparison
function isEnabled(flag: string): boolean {
  return flag === 'true';
}

app.get('/api/feature', (req, res) => {
  const isAdmin = req.query.admin === 'true';
  if (isAdmin) {
    return res.json({ secretData: 'classified' });
  }
  res.json({ publicData: 'hello' });
});

function parseConfigValue(val: string): boolean {
  return val === 'true';
}
