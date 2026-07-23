// [frensense]
// observation: Audit log records can be deleted by users or administrators without special controls.
// impact: An attacker who gains admin access can delete all traces of their activity. Without immutable audit logs, compliance with SOC2, PCI-DSS, and GDPR is impossible. Forensic investigations have no data to analyze.
// improvement: Use append-only audit log storage. Prevent deletion or modification of audit records. Consider using a separate database with restricted access or a write-only API.

app.delete('/api/admin/audit-logs/:id', async (req, res) => {
  // VULNERABLE: audit logs can be deleted
  await db.query('DELETE FROM audit_logs WHERE id = $1', [req.params.id]);
  res.json({ status: 'deleted' });
});

app.post('/api/admin/clear-audit-logs', async (req, res) => {
  // VULNERABLE: bulk delete of audit logs
  const { beforeDate } = req.body;
  await db.query('DELETE FROM audit_logs WHERE created_at < $1', [beforeDate]);
  res.json({ status: 'cleared' });
});
