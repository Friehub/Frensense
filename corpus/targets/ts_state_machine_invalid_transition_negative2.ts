// SAFE: Uses a Prisma enum-based approach with updateMany for atomic transition enforcement

enum OrderStatus {
  PENDING = 'PENDING',
  PROCESSING = 'PROCESSING',
  SHIPPED = 'SHIPPED',
  DELIVERED = 'DELIVERED',
  COMPLETED = 'COMPLETED',
  CANCELLED = 'CANCELLED',
  RETURNED = 'RETURNED',
}

const TRANSITION_MAP: Record<OrderStatus, OrderStatus[]> = {
  [OrderStatus.PENDING]: [OrderStatus.PROCESSING, OrderStatus.CANCELLED],
  [OrderStatus.PROCESSING]: [OrderStatus.SHIPPED, OrderStatus.CANCELLED],
  [OrderStatus.SHIPPED]: [OrderStatus.DELIVERED],
  [OrderStatus.DELIVERED]: [OrderStatus.COMPLETED, OrderStatus.RETURNED],
  [OrderStatus.COMPLETED]: [],
  [OrderStatus.CANCELLED]: [],
  [OrderStatus.RETURNED]: [],
};

export async function adminUpdateOrderStatus(
  prisma: PrismaClient,
  orderId: string,
  newStatus: OrderStatus
) {
  const order = await prisma.order.findUnique({
    where: { id: orderId },
    select: { status: true }
  });

  if (!order) throw new Error('Order not found');

  const allowed = TRANSITION_MAP[order.status as OrderStatus];
  if (!allowed?.includes(newStatus)) {
    throw new Error(`Invalid transition from ${order.status} to ${newStatus}`);
  }

  const updated = await prisma.order.updateMany({
    where: { id: orderId, status: order.status },
    data: { status: newStatus }
  });

  if (updated.count === 0) {
    throw new Error('Concurrent modification detected');
  }

  if (newStatus === OrderStatus.CANCELLED) {
    await issueRefund(orderId, prisma);
  }

  return { success: true };
}
