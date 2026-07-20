// SAFE: Uses Prisma with a server-side validation rule that prevents 100% discount and enforces minimum charge

export async function checkoutWithCoupon(prisma: PrismaClient, userId: string, couponCode: string) {
  const coupon = await prisma.coupon.findUnique({ where: { code: couponCode } });
  if (!coupon || !coupon.active) throw new Error('Invalid coupon');
  if (coupon.expiresAt && coupon.expiresAt < new Date()) throw new Error('Coupon expired');

  const maxPercentOff = 99;
  if (coupon.percentOff > maxPercentOff) {
    throw new Error(`Coupon discount exceeds maximum allowed ${maxPercentOff}%`);
  }

  const cart = await prisma.cart.findUnique({ where: { userId } });
  if (!cart || cart.items.length === 0) throw new Error('Cart is empty');

  const discount = cart.total * (coupon.percentOff / 100);
  const finalTotal = Math.max(cart.total - discount, 0.50);

  const order = await prisma.order.create({
    data: {
      userId,
      total: finalTotal,
      couponId: coupon.id,
      status: 'PENDING',
    },
  });

  return order;
}
