// SAFE: Uses a middleware that queries both role and status from a single DB call
async function requireActiveRole(...roles: string[]) {
  return async (req: Request, res: Response, next: NextFunction) => {
    const user = await db.prepare('SELECT role, status FROM users WHERE id = ?').bind(req.session.userId).first();
    if (!user || !roles.includes(user.role)) return res.status(403).json({ error: 'Forbidden' });
    if (user.status !== 'active' && user.status !== 'active_pending') return res.status(403).json({ error: 'Account not active' });
    next();
  };
}

app.get('/admin/dashboard', requireActiveRole('admin'), async (req, res) => {
  const metrics = await db.prepare('SELECT * FROM admin_metrics').all();
  res.json(metrics);
});
