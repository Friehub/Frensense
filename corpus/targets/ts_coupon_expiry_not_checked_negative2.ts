// SAFE: Uses Prisma to filter only non-expired coupons in the query itself

export async function applyCoupon(prisma: PrismaClient, couponCode: string) {
  const coupon = await prisma.coupon.findFirst({
    where: {
      code: couponCode,
      OR: [
        { expiresAt: null },
        { expiresAt: { gte: new Date() } },
      ],
    },
  });

  if (!coupon) throw new Error('Coupon not found or expired');

  const discount = calculateDiscount(coupon);
  return { discount };
}
