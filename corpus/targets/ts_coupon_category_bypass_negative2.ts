// SAFE: Uses Prisma with category-scoped item filtering

export async function applyCoupon(prisma: PrismaClient, userId: string, couponCode: string) {
  const coupon = await prisma.coupon.findUnique({ where: { code: couponCode } });
  if (!coupon || !coupon.active) throw new Error('Invalid coupon');

  const cartItems = await prisma.cartItem.findMany({
    where: { cart: { userId } },
    include: { product: true },
  });

  if (cartItems.length === 0) throw new Error('Cart is empty');

  let eligibleTotal = 0;
  let total = 0;

  for (const item of cartItems) {
    total += Number(item.product.price) * item.quantity;
    if (coupon.restrictedCategoryId && item.product.categoryId === coupon.restrictedCategoryId) {
      eligibleTotal += Number(item.product.price) * item.quantity;
    }
  }

  if (eligibleTotal === 0 && coupon.restrictedCategoryId) {
    throw new Error('Coupon does not apply to any items in your cart');
  }

  const discountBase = coupon.restrictedCategoryId ? eligibleTotal : total;
  const discount = Math.min(discountBase * (coupon.percentOff / 100), coupon.maxDiscount ?? Infinity);

  const finalTotal = Math.max(0, total - discount);

  await prisma.cart.update({
    where: { userId },
    data: { total: finalTotal, appliedCouponId: coupon.id },
  });

  return { finalTotal, discount };
}
