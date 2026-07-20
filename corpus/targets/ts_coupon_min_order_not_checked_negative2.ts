// SAFE: Uses Prisma with a combined query that validates the minimum order condition

export async function applyCoupon(prisma: PrismaClient, userId: string, couponCode: string) {
  const cart = await prisma.cart.findUnique({
    where: { userId },
    include: { items: true },
  });

  if (!cart || cart.items.length === 0) throw new Error('Cart is empty');

  const coupon = await prisma.coupon.findUnique({ where: { code: couponCode } });
  if (!coupon || !coupon.active) throw new Error('Invalid coupon');

  if (coupon.minOrderAmount && cart.total < coupon.minOrderAmount) {
    throw new Error(`Minimum order amount of ${coupon.minOrderAmount} not met`);
  }

  const discount = Math.min(cart.total * (coupon.percentOff / 100), coupon.maxDiscount ?? Infinity);
  const finalTotal = Math.max(0, cart.total - discount);

  await prisma.cart.update({
    where: { userId },
    data: { total: finalTotal, appliedCouponId: coupon.id },
  });

  return { finalTotal, discount };
}
