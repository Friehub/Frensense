// SAFE: Uses Prisma with type-filtered aggregation to avoid double-counting

export async function calculateRevenue(prisma: PrismaClient, userId: string) {
  const revenue = await prisma.ledger.aggregate({
    where: { userId, type: 'CREDIT' },
    _sum: { amount: true },
  });

  const expenses = await prisma.ledger.aggregate({
    where: { userId, type: 'DEBIT' },
    _sum: { amount: true },
  });

  return {
    grossRevenue: revenue._sum.amount ?? 0,
    totalExpenses: expenses._sum.amount ?? 0,
    netRevenue: (revenue._sum.amount ?? 0) - (expenses._sum.amount ?? 0),
  };
}
