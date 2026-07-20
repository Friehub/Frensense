// SAFE: Amount is calculated server-side from the cart total

import Stripe from 'stripe';
import express from 'express';

const stripe = new Stripe('sk_test_...');
const app = express();

function getCartTotal(userId: string): Promise<number> {
  return db.query('SELECT SUM(price * quantity) FROM cart_items WHERE user_id = $1', [userId]);
}

app.post('/api/create-payment-intent', async (req, res) => {
  const total = await getCartTotal(req.user.id);

  const paymentIntent = await stripe.paymentIntents.create({
    amount: total,
    currency: 'usd',
  });

  res.json({ clientSecret: paymentIntent.client_secret });
});
