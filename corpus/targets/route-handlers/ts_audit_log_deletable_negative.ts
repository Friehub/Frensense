// SAFE: audit logs are immutable — no delete endpoint
// Audit logs are stored in append-only table
// DELETE endpoint does not exist

// SAFE: if cleanup is needed, use a separate archive process
app.post('/api/admin/archive-audit-logs', async (req, res) => {
  // SAFE: logs are moved to cold storage, not deleted
  const { beforeDate } = req.body;
  await db.query(
    `INSERT INTO audit_logs_archive SELECT * FROM audit_logs WHERE created_at < $1`,
    [beforeDate]
  );
  // LOG this action in the audit log (can't delete)
  // Only DB admin with separate credentials can truncate
  res.json({ status: 'archived' });
});
