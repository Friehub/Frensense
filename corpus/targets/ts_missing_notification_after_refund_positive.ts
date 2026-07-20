// [frensense]
// observation: Refund processed successfully but the user is not notified via email, push notification, or in-app message.
// impact: User experience is degraded — the user doesn't know their refund was processed. They may submit support tickets or chargebacks, increasing operational overhead and harming trust.
// improvement: Send a notification (email, push, SMS, or in-app) after every completed refund.

async function processRefund(refundId: string, db: DB): Promise<void> {
  const refund = await db.queryOne('SELECT * FROM refunds WHERE id = $1', [refundId]);
  await db.query('UPDATE refunds SET status = $1 WHERE id = $2', ['completed', refundId]);
  // VULNERABLE: no notification sent to user
}

async function issueStoreCredit(userId: string, amount: number, db: DB): Promise<void> {
  await db.query('UPDATE wallets SET balance = balance + $1 WHERE user_id = $2', [amount, userId]);
  // VULNERABLE: no notification about store credit
}
