// SAFE: Checkout session expires after 15 minutes with configurable duration

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();
const CHECKOUT_EXPIRY_SECONDS = 900;

app.post('/api/create-checkout', async (req, res) => {
  const session = await stripe.checkout.sessions.create({
    mode: 'payment',
    line_items: [{
      price: 'price_abc123',
      quantity: 1,
    }],
    success_url: 'https://example.com/success',
    cancel_url: 'https://example.com/cancel',
    expires_at: Math.floor(Date.now() / 1000) + CHECKOUT_EXPIRY_SECONDS,
  });

  res.json({ url: session.url });
});
