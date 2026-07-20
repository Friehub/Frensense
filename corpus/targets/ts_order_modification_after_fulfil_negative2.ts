// SAFE: Uses Prisma with a state guard in the update query

export async function modifyOrder(prisma: PrismaClient, orderId: string, updates: Record<string, any>) {
  const MODIFIABLE = ['PENDING', 'PROCESSING'];

  const updated = await prisma.order.updateMany({
    where: {
      id: orderId,
      status: { in: MODIFIABLE },
    },
    data: updates,
  });

  if (updated.count === 0) {
    const order = await prisma.order.findUnique({ where: { id: orderId } });
    if (!order) throw new Error('Order not found');
    throw new Error(`Order cannot be modified in ${order.status} state`);
  }

  return { modified: true };
}
