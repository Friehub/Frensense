// SAFE: currency determined server-side
app.post('/api/create-payment', async (req, res) => {
  const order = await db.findOrder(req.body.orderId);
  // SAFE: currency from merchant config, not client
  const currency = order.merchantCurrency || 'usd';
  const amount = order.items.reduce((sum, item) => sum + item.price * item.quantity, 0);

  const paymentIntent = await stripe.paymentIntents.create({
    amount: Math.round(amount * 100),
    currency,
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});

app.post('/api/checkout', async (req, res) => {
  const user = await db.findUser(req.user.id);
  const currency = user.preferredCurrency || 'USD';
  const items = await db.resolveItems(req.body.items);
  const total = items.reduce((sum, item) => sum + item.price * item.quantity, 0);
  await createOrder(req.user.id, items, total, currency);
  res.json({ total, currency });
});
