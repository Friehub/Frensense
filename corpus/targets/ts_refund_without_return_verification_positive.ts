// [frensense]
// observation: The refund is processed immediately without verifying that the physical item has been returned or the service has been confirmed as defective.
// impact: A fraudulent user can request a refund for an item they never return, receiving their money back while keeping the product.
// improvement: Insert a return verification step that checks the return status before the refund is processed.

export async function processRefund(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // VULNERABLE: issues refund without checking if item was returned
  const refund = await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
  });

  await env.DB.prepare(
    'UPDATE orders SET status = ?, refund_id = ? WHERE id = ?'
  ).bind('REFUNDED', refund.id, orderId).run();

  return { refunded: true };
}
