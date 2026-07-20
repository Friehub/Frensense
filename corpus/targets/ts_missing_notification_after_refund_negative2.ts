// SAFE alternative: event-based notification
async function processRefund(refundId: string, db: DB): Promise<void> {
  const refund = await db.queryOne('SELECT * FROM refunds WHERE id = $1', [refundId]);
  await db.query('UPDATE refunds SET status = $1 WHERE id = $2', ['completed', refundId]);
  await eventBus.publish('refund.completed', refund);
}

// Separate handler sends notification
eventBus.on('refund.completed', async (event) => {
  await notificationService.sendEmail(event.userId, 'refund_completed', { amount: event.amount });
});
