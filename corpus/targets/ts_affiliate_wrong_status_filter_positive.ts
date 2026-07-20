// [frensense]
// observation: The commission eligibility check uses the wrong order status (COMPLETED instead of DELIVERED), so commissions are either paid too early or not at all.
// impact: Orders that are actually delivered never trigger commission payouts if the filter checks for COMPLETED but orders transition to COMPLETED only after a separate step, or vice versa.
// improvement: Use the correct order status that aligns with the business logic for when a commission should be earned.

export async function payAffiliateCommission(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ? AND status = ?'
  ).bind(orderId, 'COMPLETED').first();

  if (!order) throw new Error('Order not found or not completed');

  // VULNERABLE: checks for COMPLETED but orders become COMPLETED
  // only after the affiliate process, so commission is never paid
  const affiliate = await env.DB.prepare(
    'SELECT id FROM affiliates WHERE code = ?'
  ).bind(order.affiliate_code).first();

  if (!affiliate) throw new Error('Affiliate not found');

  const commission = Number(order.total) * 0.1;
  await env.DB.prepare(
    'UPDATE affiliates SET balance = balance + ? WHERE id = ?'
  ).bind(commission, affiliate.id).run();
}
