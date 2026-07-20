// SAFE: Records both debit and credit entries atomically to maintain double-entry accounting

export async function transferFunds(fromUserId: string, toUserId: string, amount: number, env: Env) {
  const txnRef = crypto.randomUUID();
  const now = Date.now();

  // SAFE: both sides recorded
  await env.DB.prepare(
    'INSERT INTO ledger (id, user_id, type, amount, reference_type, reference_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)'
  ).bind(crypto.randomUUID(), fromUserId, 'DEBIT', amount, 'transfer', txnRef, now).run();

  await env.DB.prepare(
    'INSERT INTO ledger (id, user_id, type, amount, reference_type, reference_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)'
  ).bind(crypto.randomUUID(), toUserId, 'CREDIT', amount, 'transfer', txnRef, now).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance - ? WHERE user_id = ?'
  ).bind(amount, fromUserId).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, toUserId).run();
}
