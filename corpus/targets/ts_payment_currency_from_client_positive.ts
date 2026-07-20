// [frensense]
// observation: Payment currency code taken from the client request body instead of set server-side.
// impact: An attacker can change the currency to one with a lower value or one that is not supported, potentially paying less for the same item or causing conversion errors in downstream financial systems.
// improvement: Set the currency server-side based on the merchant's configuration or the customer's region, never trust client-provided currency.

app.post('/api/create-payment', async (req, res) => {
  // VULNERABLE: currency from client
  const { amount, currency } = req.body;

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency, // attacker-controlled
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});

app.post('/api/checkout', async (req, res) => {
  // VULNERABLE: currency from request
  const { items, currency } = req.body;
  const total = calculateTotal(items, currency);
  await createOrder(req.user.id, items, total, currency);
  res.json({ total, currency });
});
