// SAFE: Uses Prisma with a user_id check in the coupon query itself

export async function applyCoupon(prisma: PrismaClient, userId: string, couponCode: string) {
  const coupon = await prisma.coupon.findFirst({
    where: {
      code: couponCode,
      active: true,
      OR: [
        { ownerUserId: null },
        { ownerUserId: userId },
      ],
    },
  });

  if (!coupon) throw new Error('Coupon not found or not valid for your account');

  const discount = calculateDiscount(coupon);
  await applyToCart(prisma, userId, discount, coupon.id);

  return { discount };
}
