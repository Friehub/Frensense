// SAFE: Uses Prisma with currency-specific ledger entries and wallet balances

export async function depositFunds(prisma: PrismaClient, userId: string, amount: number, currency: string) {
  return prisma.$transaction(async (tx) => {
    await tx.ledger.create({
      data: { userId, type: 'FUND', amount, currency },
    });

    const wallet = await tx.wallet.findUnique({
      where: { userId_currency: { userId, currency } },
    });

    if (wallet) {
      await tx.wallet.update({
        where: { id: wallet.id },
        data: { balance: { increment: amount } },
      });
    } else {
      await tx.wallet.create({
        data: { userId, currency, balance: amount },
      });
    }

    return { deposited: true, amount, currency };
  });
}
