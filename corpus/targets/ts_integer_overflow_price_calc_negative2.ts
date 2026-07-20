// SAFE alternative: use decimal.js for arbitrary precision
import Decimal from 'decimal.js';

function calculateLineTotal(price: number, quantity: number): Decimal {
  return new Decimal(price).times(quantity);
}

app.post('/api/checkout', async (req, res) => {
  const items = req.body.items;
  let total = new Decimal(0);

  for (const item of items) {
    const product = await db.findProduct(item.productId);
    total = total.plus(new Decimal(product.price).times(item.quantity));
  }

  await createOrder(req.user.id, items, total.toNumber());
  res.json({ total: total.toNumber() });
});
