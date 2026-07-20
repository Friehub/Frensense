// SAFE: Uses Prisma with validation constraints and the DB price source

export async function createOrder(prisma: PrismaClient, userId: string, items: { productId: string; quantity: number }[]) {
  for (const item of items) {
    if (item.quantity <= 0) throw new Error(`Invalid quantity: ${item.quantity}`);
  }

  const productIds = items.map((i) => i.productId);
  const products = await prisma.product.findMany({
    where: { id: { in: productIds } },
  });

  const productMap = new Map(products.map((p) => [p.id, p]));

  let total = 0;
  for (const item of items) {
    const product = productMap.get(item.productId);
    if (!product) throw new Error(`Product ${item.productId} not found`);
    total += Number(product.price) * item.quantity;
  }

  const order = await prisma.order.create({
    data: {
      userId,
      total,
      status: 'PENDING',
      items: {
        create: items.map((item) => ({
          productId: item.productId,
          quantity: item.quantity,
          unitPrice: productMap.get(item.productId)!.price,
        })),
      },
    },
  });

  return { orderId: order.id };
}
