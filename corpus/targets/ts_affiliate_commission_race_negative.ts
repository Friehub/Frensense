// SAFE: Checks idempotency before crediting commission to prevent double-payout

export async function creditCommission(affiliateId: string, orderId: string, amount: number, env: Env) {
  const existing = await env.DB.prepare(
    'SELECT id FROM affiliate_credits WHERE affiliate_id = ? AND order_id = ?'
  ).bind(affiliateId, orderId).first();

  if (existing) {
    throw new Error('Commission already credited for this order');
  }

  await env.DB.prepare(
    'INSERT INTO affiliate_credits (affiliate_id, order_id, amount) VALUES (?, ?, ?)'
  ).bind(affiliateId, orderId, amount).run();

  await env.DB.prepare(
    'UPDATE affiliates SET balance = balance + ? WHERE id = ?'
  ).bind(amount, affiliateId).run();
}
