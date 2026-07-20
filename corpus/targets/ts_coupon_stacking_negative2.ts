// SAFE: Uses Prisma with a stacking limit of one coupon per order

export async function applyCoupons(prisma: PrismaClient, userId: string, couponCodes: string[]) {
  if (couponCodes.length > 1) {
    throw new Error('Only one coupon can be applied per order');
  }

  const coupon = await prisma.coupon.findFirst({
    where: {
      code: { in: couponCodes },
      active: true,
    },
  });

  if (!coupon) throw new Error('No valid coupon found');

  const cart = await prisma.cart.findUnique({ where: { userId } });
  if (!cart) throw new Error('Cart not found');

  const discount = Math.min(cart.total * (coupon.percentOff / 100), coupon.maxDiscount ?? Infinity);
  const finalTotal = Math.max(0, cart.total - discount);

  await prisma.cart.update({
    where: { userId },
    data: { total: finalTotal, appliedCouponId: coupon.id },
  });

  await prisma.couponUsage.create({
    data: { couponId: coupon.id, userId },
  });

  return { finalTotal, discount };
}
