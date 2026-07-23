// [frensense]
// observation: Payment amount taken from the client request body instead of calculated server-side.
// impact: An attacker can modify the payment amount to any value before sending the request, including $0 or negative amounts. This allows purchasing items for free or even stealing money via negative amounts.
// improvement: Calculate the total amount server-side based on the items in the order, never trusting the client-provided amount.

import Stripe from 'stripe';

app.post('/api/create-payment', async (req, res) => {
  // VULNERABLE: amount from client
  const { amount, currency } = req.body;

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    // ...
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});

app.post('/api/paypal-order', async (req, res) => {
  // VULNERABLE: amount from client
  const { amount } = req.body;

  const order = await paypal.orders.create({
    intent: 'CAPTURE',
    purchase_units: [{ amount: { value: amount.toString(), currency_code: 'USD' } }],
  });

  res.json({ orderId: order.id });
});
