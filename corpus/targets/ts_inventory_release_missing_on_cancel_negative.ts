// SAFE: Releases the stock reservation when the order is cancelled

export async function cancelOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // SAFE: release reserved stock back to available
  await env.DB.prepare(
    'UPDATE products SET stock = stock + ?, reserved = reserved - ? WHERE id = ? AND reserved >= ?'
  ).bind(order.quantity, order.quantity, order.product_id, order.quantity).run();

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('CANCELLED', orderId).run();

  if (order.payment_intent) {
    await env.STRIPE.refunds.create({ payment_intent: order.payment_intent });
  }
}
