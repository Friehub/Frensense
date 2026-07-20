// SAFE: Proration is enabled so the customer is billed fairly for plan changes

import Stripe from 'stripe';

const stripe = new Stripe('sk_test_...');

async function changePlan(customerId: string, subscriptionId: string, newPriceId: string) {
  const subscription = await stripe.subscriptions.update(subscriptionId, {
    items: [{
      id: subscriptionId,
      price: newPriceId,
    }],
    proration_behavior: 'create_prorations',
  });

  return subscription;
}
