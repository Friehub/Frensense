// SAFE: Uses Prisma with a unique constraint and transactional commission crediting

export async function creditCommission(prisma: PrismaClient, affiliateId: string, orderId: string, amount: number) {
  return prisma.$transaction(async (tx) => {
    const existing = await tx.affiliateCredit.findFirst({
      where: { affiliateId, orderId },
    });

    if (existing) {
      throw new Error('Commission already credited for this order');
    }

    await tx.affiliateCredit.create({
      data: { affiliateId, orderId, amount },
    });

    await tx.affiliate.update({
      where: { id: affiliateId },
      data: { balance: { increment: amount } },
    });
  });
}
