// [frensense]
// observation: Enum-like field accepts any string value instead of only whitelisted values.
// impact: Invalid status values cause unexpected behavior in state machines, break conditional logic, or bypass access controls. For example, setting role to 'admin' when only 'user' and 'moderator' should be allowed.
// improvement: Validate enum fields against an allowlist of known values. Use TypeScript enums with runtime validation.

app.post('/api/order', async (req, res) => {
  // VULNERABLE: status accepts any value
  const { items, status } = req.body;
  await db.query('INSERT INTO orders (user_id, items, status) VALUES ($1, $2, $3)',
    [req.user.id, JSON.stringify(items), status]);
  res.json({ status: 'ok' });
});

app.post('/api/role', async (req, res) => {
  // VULNERABLE: role not validated against allowed values
  const { targetUserId, role } = req.body;
  await db.query('UPDATE users SET role = $1 WHERE id = $2', [role, targetUserId]);
  res.json({ status: 'ok' });
});
