// SAFE: validate positive quantity
app.post('/api/cart/add', async (req, res) => {
  const { productId, quantity } = req.body;
  if (!Number.isInteger(quantity) || quantity <= 0) {
    return res.status(400).json({ error: 'Quantity must be a positive integer' });
  }
  await db.query('INSERT INTO cart_items (user_id, product_id, quantity) VALUES ($1, $2, $3)',
    [req.user.id, productId, quantity]);
  res.json({ status: 'ok' });
});

app.post('/api/checkout', async (req, res) => {
  const items = await db.query('SELECT product_id, quantity FROM cart_items WHERE user_id = $1',
    [req.user.id]);
  let total = 0;
  for (const item of items) {
    if (item.quantity <= 0) {
      return res.status(400).json({ error: 'Invalid item quantity' });
    }
    const product = await db.findProduct(item.product_id);
    total += product.price * item.quantity;
  }
  await createOrder(req.user.id, items, total);
  res.json({ total });
});
