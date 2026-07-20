// SAFE alternative: normalize types at boundary
function normalizeId(id: string | number): number {
  if (typeof id === 'string') return parseInt(id, 10);
  return id;
}

app.get('/api/items/:id', async (req, res) => {
  const id = normalizeId(req.params.id);
  if (isNaN(id)) return res.status(400).json({ error: 'Invalid ID' });
  const item = allItems.find(item => item.id === id);
  res.json(item);
});
