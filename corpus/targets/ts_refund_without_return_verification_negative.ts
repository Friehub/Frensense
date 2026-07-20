// SAFE: Verifies that the item has been returned and scanned at the warehouse before issuing the refund

export async function processRefund(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // SAFE: check return status first
  const returnRecord = await env.DB.prepare(
    'SELECT * FROM returns WHERE order_id = ? AND status = ?'
  ).bind(orderId, 'RECEIVED').first();

  if (!returnRecord) {
    throw new Error('Item has not been returned yet. Please return the item first.');
  }

  const refund = await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
  });

  await env.DB.prepare(
    'UPDATE orders SET status = ?, refund_id = ? WHERE id = ?'
  ).bind('REFUNDED', refund.id, orderId).run();

  await env.DB.prepare(
    'UPDATE returns SET status = ? WHERE id = ?'
  ).bind('REFUNDED', returnRecord.id).run();

  return { refunded: true };
}
