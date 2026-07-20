// SAFE: Uses an atomic UPDATE with a WHERE guard to eliminate the race condition

export async function processRefund(orderId: string, env: Env) {
  // SAFE: atomic check-and-update in one operation
  const result = await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ? AND status = ?'
  ).bind('REFUND_PENDING', orderId, 'DELIVERED').run();

  if (result.meta.changes === 0) {
    throw new Error('Order is not eligible for refund or already processing');
  }

  await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
  });
}
