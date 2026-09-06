// SAFE: Role validated via JWT token, ignoring all URL parameters
import jwt from 'jsonwebtoken';

export function requireRole(...roles: string[]): express.Handler {
  return (req, res, next) => {
    const token = req.headers.authorization?.split(' ')[1];
    if (!token) return res.status(401).json({ error: 'Unauthorized' });
    try {
      const payload = jwt.verify(token, process.env.JWT_SECRET!) as any;
      if (!roles.includes(payload.role)) return res.status(403).json({ error: 'Forbidden' });
      req.user = payload;
      next();
    } catch { res.status(401).json({ error: 'Invalid token' }); }
  };
}

app.delete('/api/users/:id', requireRole('admin'), async (req, res) => {
  await db.prepare('DELETE FROM users WHERE id = ?').bind(req.params.id).run();
  res.json({ deleted: true });
});
