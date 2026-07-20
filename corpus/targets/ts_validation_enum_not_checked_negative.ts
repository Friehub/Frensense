// SAFE: validate against known values
import { z } from 'zod';

const ORDER_STATUSES = ['pending', 'confirmed', 'shipped', 'delivered', 'cancelled'] as const;
const ROLES = ['user', 'moderator', 'admin'] as const;

const orderSchema = z.object({
  items: z.array(z.string()),
  status: z.enum(ORDER_STATUSES),
});

app.post('/api/order', async (req, res) => {
  const result = orderSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  const { items, status } = result.data;
  await db.query('INSERT INTO orders (user_id, items, status) VALUES ($1, $2, $3)',
    [req.user.id, JSON.stringify(items), status]);
  res.json({ status: 'ok' });
});

const roleSchema = z.object({
  targetUserId: z.string().uuid(),
  role: z.enum(ROLES),
});

app.post('/api/role', async (req, res) => {
  const result = roleSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  await db.query('UPDATE users SET role = $1 WHERE id = $2', [result.data.role, result.data.targetUserId]);
  res.json({ status: 'ok' });
});
