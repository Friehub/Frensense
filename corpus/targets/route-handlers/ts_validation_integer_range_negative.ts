// SAFE: validate integer range
import { z } from 'zod';

const checkoutSchema = z.object({
  productId: z.string().uuid(),
  quantity: z.number().int().min(1).max(100),
});

app.post('/api/checkout', async (req, res) => {
  const result = checkoutSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  const { productId, quantity } = result.data;
  const product = await db.findProduct(productId);
  const total = product.price * quantity;
  await createOrder(req.user.id, productId, quantity, total);
  res.json({ total });
});

const profileSchema = z.object({
  age: z.number().int().min(0).max(150),
});

app.post('/api/profile', async (req, res) => {
  const result = profileSchema.safeParse(req.body);
  if (!result.success) return res.status(400).json({ errors: result.error.flatten() });
  await db.query('UPDATE users SET age = $1 WHERE id = $2', [result.data.age, req.user.id]);
  res.json({ status: 'ok' });
});
