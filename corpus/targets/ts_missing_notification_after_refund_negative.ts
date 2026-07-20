// SAFE: notify user after refund
import { notificationService } from './notifications';

async function processRefund(refundId: string, db: DB): Promise<void> {
  const refund = await db.queryOne('SELECT * FROM refunds WHERE id = $1', [refundId]);
  await db.query('UPDATE refunds SET status = $1 WHERE id = $2', ['completed', refundId]);
  await notificationService.sendEmail(refund.userId, 'refund_completed', {
    amount: refund.amount,
    method: refund.method,
  });
}

async function issueStoreCredit(userId: string, amount: number, db: DB): Promise<void> {
  await db.query('UPDATE wallets SET balance = balance + $1 WHERE user_id = $2', [amount, userId]);
  await notificationService.sendInApp(userId, 'store_credit_issued', { amount });
}
