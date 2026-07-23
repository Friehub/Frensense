// SAFE alternative: server-side price lookup for each item
app.post('/api/create-payment', async (req, res) => {
  const items = req.body.items as Array<{ productId: string; quantity: number }>;
  let total = 0;

  for (const item of items) {
    const product = await db.findProduct(item.productId);
    if (!product) return res.status(400).json({ error: `Product ${item.productId} not found` });
    total += product.price * item.quantity;
  }

  const paymentIntent = await stripe.paymentIntents.create({
    amount: Math.round(total * 100),
    currency: 'usd',
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
