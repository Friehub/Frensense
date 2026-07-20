// SAFE: log admin actions
import { auditLog } from './audit';

app.delete('/api/admin/users/:id', async (req, res) => {
  const user = await db.queryOne('SELECT * FROM users WHERE id = $1', [req.params.id]);
  await db.query('DELETE FROM users WHERE id = $1', [req.params.id]);
  await auditLog.write({
    actor: req.user.id,
    action: 'admin.user.delete',
    target: req.params.id,
    before: user,
    after: null,
    ip: req.ip,
  });
  res.json({ status: 'deleted' });
});

app.post('/api/admin/config', async (req, res) => {
  const old = await db.queryOne('SELECT value FROM config WHERE key = $1', [req.body.key]);
  await db.query('UPDATE config SET value = $1 WHERE key = $2', [req.body.value, req.body.key]);
  await auditLog.write({
    actor: req.user.id,
    action: 'admin.config.update',
    target: req.body.key,
    before: old?.value,
    after: req.body.value,
  });
  res.json({ status: 'updated' });
});
