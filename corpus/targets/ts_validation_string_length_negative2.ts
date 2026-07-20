// SAFE alternative: manual validation with trim
function validateString(value: unknown, min: number, max: number, field: string): string | null {
  if (typeof value !== 'string') return `${field} must be a string`;
  const trimmed = value.trim();
  if (trimmed.length < min) return `${field} must be at least ${min} characters`;
  if (trimmed.length > max) return `${field} must be at most ${max} characters`;
  return null;
}

app.post('/api/profile', async (req, res) => {
  const nameErr = validateString(req.body.name, 1, 100, 'name');
  const bioErr = req.body.bio ? validateString(req.body.bio, 0, 500, 'bio') : null;
  if (nameErr || bioErr) return res.status(400).json({ error: nameErr || bioErr });
  // ...
});
