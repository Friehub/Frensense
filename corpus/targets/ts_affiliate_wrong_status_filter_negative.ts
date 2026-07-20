// SAFE: Uses the correct status DELIVERED (the point at which commission is actually earned)

export async function payAffiliateCommission(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ? AND status = ?'
  ).bind(orderId, 'DELIVERED').first();

  if (!order) throw new Error('Order not found or not yet delivered');

  const affiliate = await env.DB.prepare(
    'SELECT id FROM affiliates WHERE code = ?'
  ).bind(order.affiliate_code).first();

  if (!affiliate) throw new Error('Affiliate not found');

  const commission = Number(order.total) * 0.1;
  await env.DB.prepare(
    'UPDATE affiliates SET balance = balance + ? WHERE id = ?'
  ).bind(commission, affiliate.id).run();

  await env.DB.prepare(
    'UPDATE orders SET commission_paid = 1 WHERE id = ?'
  ).bind(orderId).run();
}
