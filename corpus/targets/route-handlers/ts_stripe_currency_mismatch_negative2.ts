// SAFE: Client-provided currency is validated against a supported list and the account capabilities

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

const SUPPORTED_CURRENCIES = new Set(['usd', 'eur', 'gbp']);

app.post('/api/create-payment', async (req, res) => {
  const { amount, currency, orderId } = req.body;

  if (!SUPPORTED_CURRENCIES.has(currency)) {
    return res.status(400).send('Unsupported currency');
  }

  const account = await stripe.accounts.retrieve();
  if (!account.settings?.payouts?.supported_currencies?.includes(currency)) {
    return res.status(400).send('Currency not supported by this account');
  }

  const paymentIntent = await stripe.paymentIntents.create({
    amount,
    currency,
    metadata: { orderId },
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
