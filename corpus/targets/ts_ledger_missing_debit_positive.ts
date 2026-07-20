// [frensense]
// observation: A ledger entry records a CREDIT without a corresponding DEBIT in the counterparty's ledger, breaking double-entry accounting.
// impact: The accounting system becomes unbalanced; the total of all credits does not equal the total of all debits, enabling funds to be created from nothing or disappear without trace.
// improvement: Always record both sides of a transaction atomically: CREDIT for the recipient and DEBIT for the sender.

export async function transferFunds(fromUserId: string, toUserId: string, amount: number, env: Env) {
  // VULNERABLE: credits the recipient but never debits the sender
  await env.DB.prepare(
    'INSERT INTO ledger (user_id, type, amount, reference) VALUES (?, ?, ?, ?)'
  ).bind(toUserId, 'CREDIT', amount, `transfer_from_${fromUserId}`).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(amount, toUserId).run();
}
