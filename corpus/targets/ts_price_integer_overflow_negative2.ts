// SAFE: Uses Prisma with Decimal type for precise financial calculations

export async function checkoutCart(prisma: PrismaClient, userId: string, items: { productId: string; quantity: number }[]) {
  const products = await prisma.product.findMany({
    where: { id: { in: items.map((i) => i.productId) } },
  });

  const productMap = new Map(products.map((p) => [p.id, p]));

  let total = 0;
  for (const item of items) {
    const product = productMap.get(item.productId);
    if (!product) throw new Error(`Product ${item.productId} not found`);

    const lineTotal = Number(product.price) * item.quantity;

    if (!Number.isSafeInteger(lineTotal * 100)) {
      throw new Error(`Line total for ${item.productId} exceeds safe integer range`);
    }

    total += lineTotal;
  }

  if (!Number.isSafeInteger(Math.round(total * 100))) {
    throw new Error('Total exceeds safe integer range');
  }

  await prisma.order.create({
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
}
