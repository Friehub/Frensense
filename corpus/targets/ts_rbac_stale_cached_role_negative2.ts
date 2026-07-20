// SAFE: Short session TTL + periodic role refresh middleware
export async function refreshRole(req: Request, db: DB, next: NextFunction): Promise<void> {
  if (req.session?.userId) {
    const user = await db.prepare('SELECT role, status FROM users WHERE id = ?').bind(req.session.userId).first();
    if (user) {
      req.session.role = user.role;
      req.session.userStatus = user.status;
    } else {
      req.session.destroy();
    }
  }
  next();
}

app.use('/api/admin', refreshRole);

app.get('/api/admin/dashboard', async (req, res) => {
  if (req.session.role !== 'admin') return res.status(403).json({ error: 'Forbidden' });
  return handleAdminDashboard(req, res);
});
