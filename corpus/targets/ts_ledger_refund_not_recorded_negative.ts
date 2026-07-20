// SAFE: Records both the buyer credit and the seller debit in the ledger for every refund

export async function issueRefund(buyerId: string, sellerId: string, orderId: string, amount: number, env: Env) {
  const txnRef = crypto.randomUUID();
  const now = Date.now();

  // SAFE: reverse the seller's original earning
  await env.DB.prepare(
    'INSERT INTO ledger (id, user_id, type, amount, reference_type, reference_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)'
  ).bind(crypto.randomUUID(), sellerId, 'DEBIT', amount, 'refund', orderId, now).run();

  await env.DB.prepare(
    'INSERT INTO ledger (id, user_id, type, amount, reference_type, reference_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)'
  ).bind(crypto.randomUUID(), buyerId, 'CREDIT', amount, 'refund', orderId, now).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, buyerId).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance - ? WHERE user_id = ?'
  ).bind(amount, sellerId).run();
}
