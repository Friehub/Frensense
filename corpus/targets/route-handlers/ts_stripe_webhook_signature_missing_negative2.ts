// SAFE: Uses webhook secret from environment and verifies via constructEvent

import express from 'express';
import Stripe from 'stripe';

const stripe = new Stripe(process.env.STRIPE_SECRET_KEY);
const app = express();
const endpointSecret = process.env.STRIPE_WEBHOOK_SECRET;

app.post('/webhooks/stripe', express.raw({ type: 'application/json' }), async (req, res) => {
  const sig = req.headers['stripe-signature'];

  try {
    const event = stripe.webhooks.constructEvent(req.body, sig, endpointSecret);
    switch (event.type) {
      case 'payment_intent.succeeded': {
        await fulfillOrder(event.data.object.metadata.orderId);
        break;
      }
    }
  } catch (err) {
    console.error('Webhook signature verification failed:', err.message);
    return res.status(401).send('Invalid signature');
  }

  res.json({ received: true });
});
