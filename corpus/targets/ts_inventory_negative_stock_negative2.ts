// SAFE: Uses Prisma with a validation check and a gte filter in the update

export async function deductStock(prisma: PrismaClient, productId: string, quantity: number) {
  const updated = await prisma.product.updateMany({
    where: {
      id: productId,
      stock: { gte: quantity },
    },
    data: { stock: { decrement: quantity } },
  });

  if (updated.count === 0) {
    throw new Error('Insufficient stock');
  }
}
