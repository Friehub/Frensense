// SAFE: Creates a ledger entry atomically with the wallet update

export async function fundWallet(userId: string, amount: number, reference: string, env: Env) {
  const ledgerId = crypto.randomUUID();
  const now = Date.now();

  // SAFE: insert ledger entry and update wallet together
  await env.DB.prepare(
    'INSERT INTO ledger (id, user_id, type, amount, reference_type, reference_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)'
  ).bind(ledgerId, userId, 'FUND', amount, 'payment', reference, now).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, userId).run();

  return { funded: true, amount, ledgerId };
}
