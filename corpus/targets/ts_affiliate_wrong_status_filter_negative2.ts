// SAFE: Uses Prisma with a clear status mapping and configurable trigger point

const COMMISSION_TRIGGER_STATUS = 'DELIVERED';

export async function payAffiliateCommission(prisma: PrismaClient, orderId: string) {
  return prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({
      where: { id: orderId },
      include: { affiliate: true },
    });

    if (!order) throw new Error('Order not found');
    if (order.status !== COMMISSION_TRIGGER_STATUS) {
      throw new Error(`Commission can only be paid when order is ${COMMISSION_TRIGGER_STATUS}`);
    }
    if (order.commissionPaid) throw new Error('Commission already paid');

    if (order.affiliateCode) {
      const affiliate = await tx.affiliate.findUnique({ where: { code: order.affiliateCode } });
      if (!affiliate) throw new Error('Affiliate not found');

      const commission = Number(order.total) * 0.1;

      await tx.affiliate.update({
        where: { id: affiliate.id },
        data: { balance: { increment: commission } },
      });

      await tx.order.update({
        where: { id: orderId },
        data: { commissionPaid: true },
      });

      await tx.affiliateTransaction.create({
        data: {
          affiliateId: affiliate.id,
          orderId: order.id,
          amount: commission,
          type: 'commission',
        },
      });
    }
  });
}
