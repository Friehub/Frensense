// [frensense]
// observation: A refund is processed and the buyer's wallet is credited, but no corresponding debit entry is recorded in the seller's ledger, so the seller is eventually overpaid.
// impact: Sellers receive payouts that include amounts from refunded orders, causing the platform to pay out more than it collected and suffer financial loss.
// improvement: Always record a DEBIT entry in the seller's ledger when a refund is issued, reversing the original sale entry.
// cwe: CWE-841
// cvss: 7.5
// owasp: 
// severity: High

export async function issueRefund(buyerId: string, sellerId: string, orderId: string, amount: number, env: Env) {
  // VULNERABLE: refunds the buyer but does not debit the seller's ledger
  await env.DB.prepare(
    'INSERT INTO ledger (user_id, type, amount, reference) VALUES (?, ?, ?, ?)'
  ).bind(buyerId, 'CREDIT', amount, `refund_order_${orderId}`).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, buyerId).run();
}
