// SAFE: Checks that the order has not already been refunded before processing a new refund

export async function refundOrder(req: Request, env: Env) {
  const { orderId } = await req.json() as { orderId: string };

  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // SAFE: idempotency — only refund if not already refunded
  if (order.status === 'REFUNDED') {
    throw new Error('Order has already been refunded');
  }

  const refund = await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
  });

  await env.DB.prepare(
    'UPDATE orders SET status = ?, refund_id = ? WHERE id = ? AND status != ?'
  ).bind('REFUNDED', refund.id, orderId, 'REFUNDED').run();

  return { refunded: true, refundId: refund.id };
}
