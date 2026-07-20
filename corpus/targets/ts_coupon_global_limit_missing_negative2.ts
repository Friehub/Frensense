// SAFE: Uses Prisma with an atomic increment and check on the coupon's current redemption count

export async function redeemCoupon(prisma: PrismaClient, couponCode: string) {
  const coupon = await prisma.coupon.findUnique({ where: { code: couponCode } });
  if (!coupon || !coupon.active) throw new Error('Invalid coupon');

  if (coupon.maxRedemptions > 0) {
    const updated = await prisma.coupon.updateMany({
      where: {
        code: couponCode,
        currentRedemptions: { lt: coupon.maxRedemptions },
      },
      data: { currentRedemptions: { increment: 1 } },
    });

    if (updated.count === 0) {
      throw new Error('Coupon has reached its maximum number of redemptions');
    }
  }

  const discount = calculateDiscount(coupon);
  return { discount };
}
