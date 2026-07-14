// SAFE variant 6: status check is atomic inside the transaction filter
async function handlePaymentWebhook(paymentId: string, prisma: PrismaClient) {
  // SAFE: the WHERE clause makes the update conditional — if another request
  // already set status to SUCCESS, this update finds no matching row and throws,
  // preventing the duplicate fundWallet call.
  await prisma.$transaction(async (tx) => {
    const updated = await tx.payment.update({
      where: { id: paymentId, status: 'PENDING' },  // atomic check-and-update
      data: { status: 'SUCCESS' }
    });
    await fundWallet(updated.userId, updated.amount, tx);
  });
}

async function deductCreditsAtomic(userId: string, amount: number, db: D1Database) {
  // SAFE: single atomic SQL statement — check and write in one operation
  const result = await db.prepare(
    'UPDATE credits SET balance = balance - ? WHERE user_id = ? AND balance >= ?'
  ).bind(amount, userId, amount).run();
  return result.meta.changes > 0;
}
