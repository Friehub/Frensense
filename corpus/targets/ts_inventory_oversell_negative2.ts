// SAFE: Uses Prisma $transaction with stock decrement inside a serializable transaction

export async function purchaseProduct(prisma: PrismaClient, productId: string, quantity: number, userId: string) {
  return prisma.$transaction(async (tx) => {
    const product = await tx.product.findUnique({ where: { id: productId } });
    if (!product || product.stock < quantity) {
      throw new Error('Insufficient stock');
    }

    await tx.product.update({
      where: { id: productId },
      data: { stock: { decrement: quantity } },
    });

    const order = await tx.order.create({
      data: {
        userId,
        productId,
        quantity,
        status: 'PENDING',
      },
    });

    return order;
  }, { isolationLevel: 'Serializable' });
}
