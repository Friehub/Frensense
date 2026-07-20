// SAFE: Records the currency in every ledger entry and enforces currency-specific wallet updates

export async function depositFunds(userId: string, amount: number, currency: string, env: Env) {
  // SAFE: currency is always stored alongside the amount
  await env.DB.prepare(
    'INSERT INTO ledger (user_id, type, amount, currency) VALUES (?, ?, ?, ?)'
  ).bind(userId, 'FUND', amount, currency).run();

  // SAFE: update the correct currency-specific balance
  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ? AND currency = ?'
  ).bind(amount, userId, currency).run();
}
