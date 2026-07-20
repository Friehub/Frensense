// [frensense]
// observation: Domain entity status updated in the database but no domain event (order.placed, payment.confirmed, subscription.activated) is published.
// impact: Downstream systems that depend on events (notifications, audit, analytics, webhooks, cache invalidation) never fire. The state change is invisible to the rest of the system.
// improvement: Publish a domain event after every state change. Use an event bus or message queue to notify subscribers.

async function confirmOrder(orderId: string, db: DB): Promise<void> {
  // VULNERABLE: status updated but no event emitted
  await db.query('UPDATE orders SET status = $1 WHERE id = $2', ['confirmed', orderId]);
}

async function activateSubscription(subscriptionId: string, db: DB): Promise<void> {
  // VULNERABLE: status changed, no event
  await db.query('UPDATE subscriptions SET status = $1 WHERE id = $2', ['active', subscriptionId]);
}

async function processRefund(refundId: string, db: DB): Promise<void> {
  // VULNERABLE: refund processed without event
  await db.query('UPDATE refunds SET status = $1 WHERE id = $2', ['completed', refundId]);
}
