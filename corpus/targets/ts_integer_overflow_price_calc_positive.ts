// [frensense]
// observation: Price multiplied by quantity without overflow protection. JavaScript uses IEEE 754 doubles — integers above Number.MAX_SAFE_INTEGER (9,007,199,254,740,991) lose precision.
// impact: Large quantities or prices cause silent precision loss. $9,007,199,254,740,992 + 1 = $9,007,199,254,740,992 (no change). E-commerce platforms with high-volume orders or cryptocurrency microtransactions are most at risk.
// improvement: Use BigInt for financial calculations, or check against Number.MAX_SAFE_INTEGER before multiplication.

app.post('/api/checkout', async (req, res) => {
  const items = req.body.items;
  let total = 0;

  for (const item of items) {
    const product = await db.findProduct(item.productId);
    // VULNERABLE: overflow when quantity is large
    total += product.price * item.quantity;
  }

  await createOrder(req.user.id, items, total);
  res.json({ total });
});

function calculateLineTotal(price: number, quantity: number): number {
  // VULNERABLE: can overflow MAX_SAFE_INTEGER
  return price * quantity;
}
