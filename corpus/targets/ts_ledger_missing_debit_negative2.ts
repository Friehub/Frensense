// SAFE: Uses Prisma $transaction to atomically record both sides of the double-entry transfer

export async function transferFunds(prisma: PrismaClient, fromUserId: string, toUserId: string, amount: number) {
  return prisma.$transaction(async (tx) => {
    const senderWallet = await tx.wallet.findUnique({ where: { userId: fromUserId } });
    if (!senderWallet || Number(senderWallet.balance) < amount) {
      throw new Error('Insufficient funds');
    }

    await tx.wallet.update({
      where: { userId: fromUserId },
      data: { balance: { decrement: amount } },
    });

    await tx.wallet.update({
      where: { userId: toUserId },
      data: { balance: { increment: amount } },
    });

    const txnRef = crypto.randomUUID();

    await tx.ledger.create({
      data: { userId: fromUserId, type: 'DEBIT', amount, referenceType: 'transfer', referenceId: txnRef },
    });

    await tx.ledger.create({
      data: { userId: toUserId, type: 'CREDIT', amount, referenceType: 'transfer', referenceId: txnRef },
    });

    return { transferred: true, amount };
  });
}
