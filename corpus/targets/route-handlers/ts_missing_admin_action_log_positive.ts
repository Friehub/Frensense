// [frensense]
// observation: Administrator actions (user delete, config change, role modify) executed without logging.
// impact: Without audit logging, administrators can act with impunity. Malicious or accidental destructive actions cannot be traced. Compliance with SOC2, GDPR, and financial regulations requires immutable audit trails for admin operations.
// improvement: Log every administrative action: who performed it, what was changed, the previous value, and the timestamp.

app.delete('/api/admin/users/:id', async (req, res) => {
  // VULNERABLE: admin deletion not logged
  await db.query('DELETE FROM users WHERE id = $1', [req.params.id]);
  res.json({ status: 'deleted' });
});

app.post('/api/admin/config', async (req, res) => {
  // VULNERABLE: config change not logged
  await db.query('UPDATE config SET value = $1 WHERE key = $2',
    [req.body.value, req.body.key]);
  res.json({ status: 'updated' });
});
