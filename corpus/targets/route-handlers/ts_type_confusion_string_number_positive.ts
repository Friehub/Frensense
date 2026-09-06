// [frensense]
// observation: String ID compared to number ID using loose equality (==) instead of strict equality (===).
// impact: Loose equality can cause unexpected matches. '123' == 123 is true, but if the string has leading zeros ('00123') or is unexpectedly NaN, type coercion produces incorrect results. This can bypass authorization checks.
// improvement: Always use === strict equality. Convert types explicitly before comparison with parseInt() or String().

function checkOwnership(userId: string, resource: { ownerId: number }): boolean {
  // VULNERABLE: string vs number loose comparison
  return userId == resource.ownerId;
}

app.get('/api/items/:id', async (req, res) => {
  // VULNERABLE: params.id is string, item.id is number
  const item = await db.findItem(req.params.id);
  // VULNERABLE: loose comparison in filter
  const filtered = allItems.filter(item => item.id == req.params.id);
  res.json(item);
});

app.post('/api/transfer', async (req, res) => {
  // VULNERABLE: req.body.fromAccount is string, db account ID is number
  if (req.body.fromAccount == req.user.accountId) {
    // Allow transfer
  }
});
