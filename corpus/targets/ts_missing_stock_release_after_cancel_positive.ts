// [frensense]
// observation: Order cancelled but the reserved inventory is not released back to available stock.
// impact: Each cancellation permanently consumes inventory. Over time, phantom stock disappears, causing overselling even though physical inventory is available. Lost revenue and customer complaints about unavailable items.
// improvement: Always call releaseStock() or decrement reserved + increment available when an order is cancelled.

async function cancelOrder(orderId: string, db: DB): Promise<void> {
  // VULNERABLE: order cancelled, inventory never restored
  await db.query('UPDATE orders SET status = $1 WHERE id = $2', ['cancelled', orderId]);
}

async function cancelSubscription(subscriptionId: string, db: DB): Promise<void> {
  // VULNERABLE: no resource deallocation
  await db.query('UPDATE subscriptions SET status = $1 WHERE id = $2', ['cancelled', subscriptionId]);
  // License seat not released
}
