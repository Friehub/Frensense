// SAFE: release inventory on cancellation
async function cancelOrder(orderId: string, db: DB): Promise<void> {
  const items = await db.query('SELECT product_id, quantity FROM order_items WHERE order_id = $1', [orderId]);
  await db.transaction(async (tx) => {
    await tx.query('UPDATE orders SET status = $1 WHERE id = $2', ['cancelled', orderId]);
    for (const item of items) {
      await tx.query(
        'UPDATE inventory SET reserved = reserved - $1, available = available + $1 WHERE product_id = $2',
        [item.quantity, item.product_id]
      );
    }
  });
}

async function cancelSubscription(subscriptionId: string, db: DB): Promise<void> {
  await db.transaction(async (tx) => {
    const sub = await tx.queryOne('SELECT user_id, plan FROM subscriptions WHERE id = $1', [subscriptionId]);
    await tx.query('UPDATE subscriptions SET status = $1 WHERE id = $2', ['cancelled', subscriptionId]);
    await tx.query('UPDATE licenses SET assigned = false WHERE user_id = $1', [sub.user_id]);
  });
}
