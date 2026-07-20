// SAFE: create ledger entry for every wallet operation
async function creditWallet(userId: string, amount: number, db: DB): Promise<void> {
  await db.transaction(async (tx) => {
    await tx.query('UPDATE wallets SET balance = balance + $1 WHERE user_id = $2', [amount, userId]);
    await tx.query(
      'INSERT INTO ledger (user_id, type, amount, created_at) VALUES ($1, $2, $3, NOW())',
      [userId, 'credit', amount]
    );
  });
}

async function debitWallet(userId: string, amount: number, db: DB): Promise<void> {
  await db.transaction(async (tx) => {
    await tx.query('UPDATE wallets SET balance = balance - $1 WHERE user_id = $2', [amount, userId]);
    await tx.query(
      'INSERT INTO ledger (user_id, type, amount, created_at) VALUES ($1, $2, $3, NOW())',
      [userId, 'debit', amount]
    );
  });
}
