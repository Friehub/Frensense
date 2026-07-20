// SAFE: use BigInt for financial calculations
app.post('/api/checkout', async (req, res) => {
  const items = req.body.items;
  let total = 0n;

  for (const item of items) {
    const product = await db.findProduct(item.productId);
    const price = BigInt(Math.round(product.price * 100)); // cents
    const quantity = BigInt(item.quantity);
    total += price * quantity;
  }

  // SAFE: convert back to decimal for response
  const totalNumber = Number(total) / 100;
  await createOrder(req.user.id, items, totalNumber);
  res.json({ total: totalNumber });
});

function calculateLineTotal(price: number, quantity: number): number {
  const MAX_SAFE = Number.MAX_SAFE_INTEGER;
  if (price > MAX_SAFE / quantity) {
    throw new Error('Price calculation overflow');
  }
  return price * quantity;
}
