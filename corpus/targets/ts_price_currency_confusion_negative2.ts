// SAFE: Rejects multi-currency carts and enforces a single currency per transaction

export async function checkout(prisma: PrismaClient, userId: string, items: { productId: string; quantity: number }[]) {
  const products = await prisma.product.findMany({
    where: { id: { in: items.map((i) => i.productId) } },
  });

  const currencies = new Set(products.map((p) => p.currency));
  if (currencies.size > 1) {
    throw new Error('All items must be in the same currency');
  }

  const currency = products[0].currency;
  let total = 0;

  for (const item of items) {
    const product = products.find((p) => p.id === item.productId);
    if (!product) throw new Error(`Product ${item.productId} not found`);
    total += Number(product.price) * item.quantity;
  }

  await prisma.order.create({
    data: {
      userId,
      total,
      currency,
      status: 'PENDING',
      items: {
        create: items.map((item) => ({
          productId: item.productId,
          quantity: item.quantity,
          unitPrice: products.find((p) => p.id === item.productId)!.price,
        })),
      },
    },
  });
}
