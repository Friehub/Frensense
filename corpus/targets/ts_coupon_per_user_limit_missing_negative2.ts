// SAFE: Uses Prisma with unique constraint and check on usage count

export async function applyCoupon(prisma: PrismaClient, userId: string, couponCode: string) {
  const coupon = await prisma.coupon.findUnique({ where: { code: couponCode } });
  if (!coupon || !coupon.active) throw new Error('Invalid coupon');

  if (coupon.perUserLimit > 0) {
    const usageCount = await prisma.couponUsage.count({
      where: { couponId: coupon.id, userId },
    });

    if (usageCount >= coupon.perUserLimit) {
      throw new Error('Coupon usage limit reached for this user');
    }
  }

  await prisma.$transaction(async (tx) => {
    await tx.couponUsage.create({
      data: { couponId: coupon.id, userId },
    });

    await applyDiscountToCart(tx, userId, coupon);
  });
}
