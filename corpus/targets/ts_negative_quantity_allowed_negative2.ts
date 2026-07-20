// SAFE alternative: use zod for input validation
import { z } from 'zod';

const cartSchema = z.object({
  productId: z.string().uuid(),
  quantity: z.number().int().positive(),
});

app.post('/api/cart/add', async (req, res) => {
  const result = cartSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  await db.query('INSERT INTO cart_items (user_id, product_id, quantity) VALUES ($1, $2, $3)',
    [req.user.id, result.data.productId, result.data.quantity]);
  res.json({ status: 'ok' });
});
