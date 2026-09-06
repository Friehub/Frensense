// SAFE: Middleware-based RBAC enforces checks before any handler runs
function authorize(...roles: string[]): express.Handler {
  return (req, res, next) => {
    if (!req.session || !roles.includes(req.session.role)) {
      return res.status(403).json({ error: 'Forbidden' });
    }
    next();
  };
}

const adminOnly = authorize('admin');
app.delete('/api/users/:id', adminOnly, async (req, res) => {
  await db.prepare('DELETE FROM users WHERE id = ?').bind(req.params.id).run();
  res.json({ deleted: true });
});

const supportOrAdmin = authorize('admin', 'support');
app.post('/api/refunds', supportOrAdmin, async (req, res) => {
  const { orderId, amount } = req.body;
  const refund = await db.prepare('INSERT INTO refunds (order_id, amount, status) VALUES (?, ?, ?)').bind(orderId, amount, 'pending').run();
  res.json({ refundId: refund.id });
});
