// [frensense]
// observation: The wallet balance is updated but no corresponding ledger entry is created, breaking the double-entry accounting invariant.
// impact: Financial reconciliation becomes impossible; an attacker can manipulate the wallet balance without a traceable record, enabling undetected theft or accounting fraud.
// improvement: Always create a ledger entry atomically with every wallet mutation, recording the type, amount, reference, and timestamp.

export async function fundWallet(userId: string, amount: number, env: Env) {
  // VULNERABLE: wallet funded without a corresponding ledger entry
  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, userId).run();

  return { funded: true, amount };
}
