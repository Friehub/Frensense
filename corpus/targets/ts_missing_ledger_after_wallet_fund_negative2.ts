// SAFE: Uses Prisma in a transaction to atomically create the ledger entry and update the wallet

export async function fundWallet(prisma: PrismaClient, userId: string, amount: number, reference: string) {
  return prisma.$transaction(async (tx) => {
    const ledger = await tx.ledger.create({
      data: {
        userId,
        type: 'FUND',
        amount,
        referenceType: 'payment',
        referenceId: reference,
      },
    });

    await tx.wallet.update({
      where: { userId },
      data: { balance: { increment: amount } },
    });

    return { funded: true, amount, ledgerId: ledger.id };
  });
}
