// [frensense]
// observation: The affiliate commission for an order can be credited twice because there is no idempotency check or unique constraint on the transaction.
// impact: An attacker can trigger the commission payout endpoint twice for the same order, earning double the commission.
// improvement: Add an idempotency check (e.g., commission_paid flag) or a unique constraint on (affiliate_id, order_id).
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

export async function creditCommission(affiliateId: string, orderId: string, amount: number, env: Env) {
  // VULNERABLE: no idempotency — commission can be credited twice
  await env.DB.prepare(
    'INSERT INTO affiliate_credits (affiliate_id, order_id, amount) VALUES (?, ?, ?)'
  ).bind(affiliateId, orderId, amount).run();

  await env.DB.prepare(
    'UPDATE affiliates SET balance = balance + ? WHERE id = ?'
  ).bind(amount, affiliateId).run();
}
