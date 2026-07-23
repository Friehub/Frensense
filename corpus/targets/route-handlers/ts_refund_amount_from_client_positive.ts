// [frensense]
// observation: The refund amount is taken directly from the client request body rather than being calculated server-side from the original payment amount.
// impact: A malicious user can request a refund for more than they originally paid, stealing money from the merchant.
// improvement: Always use the original payment amount from the database as the refund amount, and never trust the client-provided refund amount.

export async function requestRefund(req: Request, env: Env) {
  const { orderId, amount } = await req.json() as { orderId: string; amount: number };

  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // VULNERABLE: uses client-provided amount, not the original payment value
  const refund = await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
    amount: Math.round(amount * 100),
  });

  await env.DB.prepare(
    'UPDATE orders SET status = ?, refund_amount = ? WHERE id = ?'
  ).bind('REFUNDED', amount, orderId).run();

  return { refunded: true };
}
