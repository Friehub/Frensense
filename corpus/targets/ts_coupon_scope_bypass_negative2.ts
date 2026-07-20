// SAFE: An alternate fixvariant using Prisma that also validates scope and category restrictions

export async function applyDiscount(prisma: PrismaClient, cartId: string, couponCode: string) {
  const coupon = await prisma.coupon.findUnique({ where: { code: couponCode } });
  if (!coupon || new Date() > coupon.expiresAt) {
    throw new Error('Invalid or expired coupon');
  }

  const cart = await prisma.cart.findUnique({
    where: { id: cartId },
    include: { items: { include: { product: true } } },
  });

  if (!cart || cart.items.length === 0) throw new Error('Cart is empty');

  let eligibleTotal = 0;
  for (const item of cart.items) {
    if (coupon.restrictedSellerId && item.product.sellerId !== coupon.restrictedSellerId) continue;
    if (coupon.restrictedCategory && item.product.category !== coupon.restrictedCategory) continue;
    eligibleTotal += Number(item.product.price) * item.quantity;
  }

  if (eligibleTotal === 0 && (coupon.restrictedSellerId || coupon.restrictedCategory)) {
    throw new Error('Coupon does not apply to any items in the cart');
  }

  const cartTotal = cart.items.reduce(
    (sum, item) => sum + Number(item.product.price) * item.quantity, 0
  );

  const discount = Math.min(eligibleTotal * (coupon.percentOff / 100), coupon.maxDiscount ?? Infinity);
  const finalTotal = Math.max(0, cartTotal - discount);

  await prisma.cart.update({
    where: { id: cartId },
    data: { total: finalTotal, appliedCouponId: coupon.id },
  });

  return { newTotal: finalTotal, discount };
}
