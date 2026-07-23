// [frensense]
// observation: The refund endpoint has no idempotency guard, allowing the same order to be refunded multiple times by replaying the same request.
// impact: An attacker can call the refund endpoint repeatedly to drain the merchant's Stripe balance, receiving multiple refunds for a single payment.
// improvement: Add an idempotency key check or a refund status check before processing the refund.

export async function refundOrder(req: Request, env: Env) {
  const { orderId } = await req.json() as { orderId: string };

  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // VULNERABLE: no check if already refunded; can be called many times
  const refund = await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
  });

  await env.DB.prepare(
    'UPDATE orders SET status = ?, refund_id = ? WHERE id = ?'
  ).bind('REFUNDED', refund.id, orderId).run();

  return { refunded: true, refundId: refund.id };
}
