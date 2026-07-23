// SAFE alternative: express-validator
import { body, validationResult } from 'express-validator';

app.post('/api/checkout',
  body('productId').isUUID(),
  body('quantity').isInt({ min: 1, max: 100 }),
  async (req, res) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
    const product = await db.findProduct(req.body.productId);
    const total = product.price * req.body.quantity;
    await createOrder(req.user.id, req.body.productId, req.body.quantity, total);
    res.json({ total });
  }
);
