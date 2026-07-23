// SAFE: Uses shared middleware for CRUD authorization
function requireRole(...roles: string[]): express.Handler {
  return (req, res, next) => {
    if (!req.session || !roles.includes(req.session.role)) return res.status(403).json({ error: 'Forbidden' });
    next();
  };
}

const authorOrAdmin = requireRole('author', 'admin');

app.post('/api/articles', authorOrAdmin, async (req, res) => {
  await db.prepare('INSERT INTO articles (title, content, author_id) VALUES (?, ?, ?)').bind(req.body.title, req.body.content, req.session.userId).run();
  res.status(201).json({ created: true });
});

app.put('/api/articles/:id', authorOrAdmin, async (req, res) => {
  const result = await db.prepare('UPDATE articles SET title = ?, content = ? WHERE id = ? AND author_id = ?').bind(req.body.title, req.body.content, req.params.id, req.session.userId).run();
  if (!result) return res.status(404).json({ error: 'Not found' });
  res.json({ updated: true });
});

app.delete('/api/articles/:id', authorOrAdmin, async (req, res) => {
  const result = await db.prepare('DELETE FROM articles WHERE id = ? AND author_id = ?').bind(req.params.id, req.session.userId).run();
  if (!result) return res.status(404).json({ error: 'Not found' });
  res.json({ deleted: true });
});
