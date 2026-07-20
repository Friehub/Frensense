// [frensense]
// observation: "When a subscription plan is changed, proration is not calculated, causing the customer to be overcharged or undercharged for the billing period."
// impact: "Customers may be billed for the full new plan immediately without credit for the unused portion of the old plan, leading to billing complaints and potential chargebacks."
// improvement: "Always set proration_behavior to 'create_prorations' on subscription updates and consider the invoice that will be generated."

import Stripe from 'stripe';

const stripe = new Stripe('sk_test_...');

async function changePlan(customerId: string, subscriptionId: string, newPriceId: string) {
  const subscription = await stripe.subscriptions.update(subscriptionId, {
    items: [{
      id: subscriptionId,
      price: newPriceId,
    }],
  });

  return subscription;
}

async function upgradePlan(customerId: string, subscriptionId: string, newPriceId: string) {
  const subscription = await stripe.subscriptions.update(subscriptionId, {
    items: [{
      id: subscriptionId,
      price: newPriceId,
    }],
    proration_behavior: 'none',
  });

  return subscription;
}
