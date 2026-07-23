// SAFE: DELETE endpoint is consistently protected with authentication
const app = express();

function authenticate(req: Request, res: Response, next: NextFunction): void {
  const token = req.headers.authorization?.split(' ')[1];
  if (!token) { res.status(401).json({ error: 'Unauthorized' }); return; }
  try {
    req.user = jwt.verify(token, process.env.JWT_SECRET!);
    next();
  } catch { res.status(401).json({ error: 'Invalid token' }); }
}

app.get('/api/items/:id', authenticate, async (req, res) => {
  const item = await db.prepare('SELECT * FROM items WHERE id = ?').bind(req.params.id).first();
  res.json(item);
});

app.put('/api/items/:id', authenticate, async (req, res) => {
  await db.prepare('UPDATE items SET name = ? WHERE id = ?').bind(req.body.name, req.params.id).run();
  res.json({ updated: true });
});

app.delete('/api/items/:id', authenticate, async (req, res) => {
  await db.prepare('DELETE FROM items WHERE id = ?').bind(req.params.id).run();
  res.json({ deleted: true });
});
