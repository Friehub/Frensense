// [frensense]
// observation: When a refund is processed, the wallet is credited but no corresponding ledger entry is created, breaking the double-entry accounting system.
// impact: The platform's financial records become inconsistent — the wallet balance does not match the ledger sum, enabling fraud to go undetected and preventing accurate reconciliation.
// improvement: Always insert a ledger transaction entry for every wallet credit or debit, ensuring the double-entry invariant is maintained.

export async function issueRefund(userId: string, orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT total, stripe_pi FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // VULNERABLE: credits wallet but does not create a ledger entry
  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(Number(order.total), userId).run();

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('REFUNDED', orderId).run();
}
