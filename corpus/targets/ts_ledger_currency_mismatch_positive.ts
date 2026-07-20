// [frensense]
// observation: Ledger entries store amounts without recording the currency, and wallet balances in different currencies are treated as interchangeable, causing silent value mismatch.
// impact: A user can exploit the currency confusion to gain value — e.g., depositing 100 NGN and withdrawing 100 USD — by exploiting the missing currency field in ledger entries.
// improvement: Always store the currency alongside the ledger amount and enforce currency matching on all operations.

export async function depositFunds(userId: string, amount: number, currency: string, env: Env) {
  // VULNERABLE: ledger entry does not record currency
  await env.DB.prepare(
    'INSERT INTO ledger (user_id, type, amount) VALUES (?, ?, ?)'
  ).bind(userId, 'FUND', amount).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, userId).run();
}
