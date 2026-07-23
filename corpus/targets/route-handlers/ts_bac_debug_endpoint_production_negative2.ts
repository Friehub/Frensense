// SAFE: Debug endpoints are protected by admin authentication
import express from 'express';

const app = express();

function requireAdmin(req: Request, res: Response, next: NextFunction): void {
  if (req.session?.role !== 'admin') {
    res.status(403).json({ error: 'Forbidden' });
    return;
  }
  next();
}

app.get('/api/config', requireAdmin, (req, res) => {
  res.json({ version: '1.0.0' });
});

app.get('/api/metrics', requireAdmin, async (req, res) => {
  const metrics = await db.prepare('SELECT COUNT(*) as users FROM users').first();
  res.json(metrics);
});
