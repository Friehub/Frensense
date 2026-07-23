// SAFE: Internal API requires service-to-service auth token and network restriction
import express from 'express';

const app = express();

async function requireInternalAuth(req: Request, res: Response, next: NextFunction): Promise<void> {
  const token = req.headers['x-internal-token'];
  if (token !== process.env.INTERNAL_SECRET) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  next();
}

app.use('/internal', requireInternalAuth);

app.get('/internal/users/:id/profile', async (req, res) => {
  const user = await db.prepare('SELECT * FROM users WHERE id = ?').bind(req.params.id).first();
  res.json(user);
});

app.post('/internal/orders/sync', async (req, res) => {
  const { orders } = req.body;
  for (const order of orders) {
    await db.prepare('INSERT INTO orders (id, user_id, total) VALUES (?, ?, ?)').bind(order.id, order.userId, order.total).run();
  }
  res.json({ synced: orders.length });
});
