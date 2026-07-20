// SAFE: Always uses the original payment amount from the database for the refund

export async function requestRefund(req: Request, env: Env) {
  const { orderId } = await req.json() as { orderId: string };

  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');
  if (order.status === 'REFUNDED') throw new Error('Already refunded');

  // SAFE: refund amount comes from the original order total, not the client
  const refund = await env.STRIPE.refunds.create({
    payment_intent: order.stripe_pi,
    amount: Math.round(Number(order.total) * 100),
  });

  await env.DB.prepare(
    'UPDATE orders SET status = ?, refund_amount = ? WHERE id = ?'
  ).bind('REFUNDED', order.total, orderId).run();

  return { refunded: true, amount: order.total };
}
