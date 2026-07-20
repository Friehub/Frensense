// SAFE: use strict equality ===
function checkOwnership(userId: string, resource: { ownerId: number }): boolean {
  return userId === String(resource.ownerId);
}

app.get('/api/items/:id', async (req, res) => {
  const itemId = parseInt(req.params.id, 10);
  if (isNaN(itemId)) return res.status(400).json({ error: 'Invalid ID' });
  const filtered = allItems.filter(item => item.id === itemId);
  res.json(filtered);
});

app.post('/api/transfer', async (req, res) => {
  if (String(req.body.fromAccount) === String(req.user.accountId)) {
    // Safe comparison after explicit conversion
  }
});
