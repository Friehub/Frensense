// [frensense]
// observation: Wallet balance is updated but no corresponding ledger entry is created, breaking double-entry accounting.
// impact: Without ledger entries, financial reconciliation is impossible. If a wallet update fails silently or is reversed, the ledger shows no record of the transaction. Audits fail, and fraud detection has no data trail.
// improvement: Always create a corresponding ledger entry for every wallet credit or debit operation.

async function creditWallet(userId: string, amount: number, db: DB): Promise<void> {
  // VULNERABLE: wallet credited without ledger entry
  await db.query('UPDATE wallets SET balance = balance + $1 WHERE user_id = $2', [amount, userId]);
}

async function debitWallet(userId: string, amount: number, db: DB): Promise<void> {
  // VULNERABLE: wallet debited without ledger entry
  await db.query('UPDATE wallets SET balance = balance - $1 WHERE user_id = $2', [amount, userId]);
}
