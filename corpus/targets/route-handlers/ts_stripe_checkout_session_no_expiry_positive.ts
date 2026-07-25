// [frensense]
// observation: "Checkout Session is created without an expires_at parameter, using Stripe's default 24-hour expiry."
// impact: "A stale checkout link can be used up to 24 hours later, creating a wider window for price changes, session hijacking, or outdated inventory commitments."
// improvement: "Set a shorter expires_at value appropriate for your use case (e.g., 30 minutes)."
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

app.post('/api/create-checkout', async (req, res) => {
  const session = await stripe.checkout.sessions.create({
    mode: 'payment',
    line_items: [{
      price: 'price_abc123',
      quantity: 1,
    }],
    success_url: 'https://example.com/success',
    cancel_url: 'https://example.com/cancel',
  });

  res.json({ url: session.url });
});
