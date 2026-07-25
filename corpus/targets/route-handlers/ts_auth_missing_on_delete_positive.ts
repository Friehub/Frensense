// [frensense]
// observation: GET and PUT endpoints are protected by authentication middleware, but the DELETE endpoint for the same resource is not.
// impact: An unauthenticated attacker can delete resources even though read and update operations are protected, creating a gap in the authorization coverage.
// improvement: Apply consistent authentication and authorization checks to all HTTP methods (GET, POST, PUT, PATCH, DELETE) for every resource.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

const app = express();

app.get('/api/items/:id', authenticate, async (req, res) => {
  const item = await db.prepare('SELECT * FROM items WHERE id = ?').bind(req.params.id).first();
  res.json(item);
});

app.put('/api/items/:id', authenticate, async (req, res) => {
  await db.prepare('UPDATE items SET name = ? WHERE id = ?').bind(req.body.name, req.params.id).run();
  res.json({ updated: true });
});

app.delete('/api/items/:id', async (req, res) => {
  await db.prepare('DELETE FROM items WHERE id = ?').bind(req.params.id).run();
  res.json({ deleted: true });
});
