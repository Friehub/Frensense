// SAFE alternative: Set-based validation
const VALID_ROLES = new Set(['user', 'moderator', 'admin']);
const VALID_STATUSES = new Set(['pending', 'confirmed', 'shipped', 'delivered', 'cancelled']);

app.post('/api/role', async (req, res) => {
  const { role } = req.body;
  if (!VALID_ROLES.has(role)) return res.status(400).json({ error: `Invalid role. Must be one of: ${[...VALID_ROLES].join(', ')}` });
  await db.query('UPDATE users SET role = $1 WHERE id = $2', [role, req.body.targetUserId]);
  res.json({ status: 'ok' });
});
