// SAFE alternative: wallet service always creates ledger
class WalletService {
  async credit(userId: string, amount: number, reason: string): Promise<void> {
    const result = await db.query(
      `WITH wallet_update AS (
        UPDATE wallets SET balance = balance + $1 WHERE user_id = $2 RETURNING balance
      )
      INSERT INTO ledger (user_id, type, amount, reason, created_at)
      VALUES ($2, 'credit', $1, $3, NOW())
      RETURNING id`,
      [amount, userId, reason]
    );
  }
}
