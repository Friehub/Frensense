// [frensense]
// observation: Quantity or count field accepts negative values, allowing price manipulation or balance inversion.
// impact: A negative quantity in checkout calculates a negative total, giving the user money instead of charging them. A negative item count can also cause SQL constraint violations, leading to application errors.
// improvement: Validate that quantity is a positive integer (> 0) before processing.

app.post('/api/cart/add', async (req, res) => {
  // VULNERABLE: negative quantity accepted
  const { productId, quantity } = req.body;
  await db.query('INSERT INTO cart_items (user_id, product_id, quantity) VALUES ($1, $2, $3)',
    [req.user.id, productId, quantity]);
  res.json({ status: 'ok' });
});

app.post('/api/checkout', async (req, res) => {
  // VULNERABLE: quantity could be negative from cart
  const items = await db.query('SELECT product_id, quantity FROM cart_items WHERE user_id = $1',
    [req.user.id]);
  let total = 0;
  for (const item of items) {
    const product = await db.findProduct(item.product_id);
    // VULNERABLE: negative quantity reduces total
    total += product.price * item.quantity;
  }
  await createOrder(req.user.id, items, total);
  res.json({ total });
});
