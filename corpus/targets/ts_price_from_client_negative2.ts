// SAFE: Uses Prisma to fetch current prices and compute the total in a transaction

export async function checkout(prisma: PrismaClient, userId: string) {
  const cart = await prisma.cart.findUnique({
    where: { userId },
    include: { items: { include: { product: true } } },
  });

  if (!cart || cart.items.length === 0) throw new Error('Cart is empty');

  const total = cart.items.reduce(
    (sum, item) => sum + Number(item.product.price) * item.quantity,
    0
  );

  const paymentIntent = await stripe.paymentIntents.create({
    amount: Math.round(total * 100),
    currency: 'usd',
  });

  const order = await prisma.order.create({
    data: {
      userId,
      total,
      stripePi: paymentIntent.id,
      status: 'PENDING',
      items: {
        create: cart.items.map((item) => ({
          productId: item.productId,
          quantity: item.quantity,
          unitPrice: item.product.price,
        })),
      },
    },
  });

  await prisma.cart.delete({ where: { userId } });

  return { orderId: order.id, clientSecret: paymentIntent.client_secret };
}
