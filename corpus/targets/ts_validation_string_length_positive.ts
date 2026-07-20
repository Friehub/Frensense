// [frensense]
// observation: String input accepted without length validation, allowing excessively long values.
// impact: Very long strings consume disproportionate storage, cause UI layout issues, degrade query performance, and can be used for denial of service. Database varchar limits may be exceeded silently (truncation).
// improvement: Set explicit minimum and maximum length constraints on all string inputs.

app.post('/api/profile', async (req, res) => {
  // VULNERABLE: name and bio have no length limits
  const { name, bio } = req.body;
  await db.query('UPDATE users SET name = $1, bio = $2 WHERE id = $3', [name, bio, req.user.id]);
  res.json({ status: 'ok' });
});

app.post('/api/comments', async (req, res) => {
  // VULNERABLE: comment body has no length limit
  const { postId, body } = req.body;
  await db.query('INSERT INTO comments (post_id, author_id, body) VALUES ($1, $2, $3)',
    [postId, req.user.id, body]);
  res.json({ status: 'ok' });
});
