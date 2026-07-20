// [frensense]
// observation: When an order is cancelled, the stock reservation is not released, causing the reserved quantity to remain unavailable for other customers.
// impact: Cancelled orders permanently tie up inventory, eventually causing all stock to appear reserved and blocking legitimate purchases.
// improvement: Always restore the reserved stock quantity when an order transitions to CANCELLED.

export async function cancelOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('CANCELLED', orderId).run();

  // VULNERABLE: stock reservation is never released
  // The reserved inventory remains locked forever

  if (order.payment_intent) {
    await env.STRIPE.refunds.create({ payment_intent: order.payment_intent });
  }
}
