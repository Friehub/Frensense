// SAFE: validate URL with URL constructor + protocol allowlist
function validateUrl(url: string): string | null {
  try {
    const parsed = new URL(url);
    if (!['http:', 'https:'].includes(parsed.protocol)) return null;
    return parsed.href;
  } catch {
    return null;
  }
}

app.post('/api/profile', async (req, res) => {
  const website = validateUrl(req.body.website);
  if (!website) return res.status(400).json({ error: 'Invalid URL' });
  await db.query('UPDATE users SET website = $1 WHERE id = $2', [website, req.user.id]);
  res.json({ status: 'ok' });
});
