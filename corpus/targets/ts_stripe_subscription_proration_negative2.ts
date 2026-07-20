// SAFE: Proration is calculated and the upcoming invoice is previewed before applying the change

import Stripe from 'stripe';

const stripe = new Stripe('sk_test_...');

async function changePlan(customerId: string, subscriptionId: string, newPriceId: string) {
  const upcoming = await stripe.invoices.retrieveUpcoming({
    customer: customerId,
    subscription: subscriptionId,
    subscription_items: [{
      id: subscriptionId,
      price: newPriceId,
    }],
    subscription_proration_behavior: 'create_prorations',
  });

  const subscription = await stripe.subscriptions.update(subscriptionId, {
    items: [{
      id: subscriptionId,
      price: newPriceId,
    }],
    proration_behavior: 'create_prorations',
  });

  return { subscription, prorationAmount: upcoming.amount_due };
}
