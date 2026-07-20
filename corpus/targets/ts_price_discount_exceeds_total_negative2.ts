// SAFE: Uses Prisma with validation that the discounted total stays non-negative

export async function applyPromoCode(prisma: PrismaClient, cartId: string, code: string) {
  const cart = await prisma.cart.findUnique({ where: { id: cartId } });
  if (!cart) throw new Error('Cart not found');

  const promo = await prisma.promotion.findUnique({ where: { code } });
  if (!promo || !promo.active) throw new Error('Invalid promo code');
  if (promo.expiresAt && promo.expiresAt < new Date()) throw new Error('Promo expired');

  const discount = Math.min(
    cart.total * (promo.percentOff / 100),
    promo.maxDiscount ?? Infinity
  );

  const finalTotal = Math.max(0, cart.total - discount);

  await prisma.cart.update({
    where: { id: cartId },
    data: {
      appliedPromoId: promo.id,
      discount,
      total: finalTotal,
    },
  });

  return { finalTotal, discount };
}
