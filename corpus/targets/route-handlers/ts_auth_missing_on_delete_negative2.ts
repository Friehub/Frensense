// SAFE: Uses route-level middleware for consistent auth across all methods
function requireAuth(req: Request, res: Response, next: NextFunction): void {
  if (!req.session?.userId) return res.status(401).json({ error: 'Unauthorized' });
  next();
}

const itemsRouter = express.Router({ mergeParams: true });
itemsRouter.use(requireAuth);

itemsRouter.get('/:id', async (req, res) => {
  const item = await db.prepare('SELECT * FROM items WHERE id = ? AND user_id = ?').bind(req.params.id, req.session.userId).first();
  res.json(item);
});

itemsRouter.put('/:id', async (req, res) => {
  await db.prepare('UPDATE items SET name = ? WHERE id = ? AND user_id = ?').bind(req.body.name, req.params.id, req.session.userId).run();
  res.json({ updated: true });
});

itemsRouter.delete('/:id', async (req, res) => {
  await db.prepare('DELETE FROM items WHERE id = ? AND user_id = ?').bind(req.params.id, req.session.userId).run();
  res.json({ deleted: true });
});

const app = express();
app.use('/api/items', itemsRouter);
