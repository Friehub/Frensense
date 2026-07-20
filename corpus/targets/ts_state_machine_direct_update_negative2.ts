// SAFE: Uses Prisma updateMany with a status guard and explicit transition table for state machine enforcement

const ORDER_TRANSITIONS: Record<string, string[]> = {
  PENDING: ['PROCESSING', 'CANCELLED'],
  PROCESSING: ['SHIPPED', 'CANCELLED'],
  SHIPPED: ['DELIVERED'],
  DELIVERED: ['COMPLETED'],
  COMPLETED: [],
  CANCELLED: [],
};

export async function cancelOrder(prisma: PrismaClient, orderId: string) {
  const order = await prisma.order.findUnique({
    where: { id: orderId },
    select: { status: true, paymentId: true },
  });

  if (!order) throw new Error('Not found');

  if (!ORDER_TRANSITIONS[order.status]?.includes('CANCELLED')) {
    throw new Error(`Order in state ${order.status} cannot be cancelled`);
  }

  const updated = await prisma.order.updateMany({
    where: { id: orderId, status: order.status },
    data: { status: 'CANCELLED' },
  });

  if (updated.count === 0) {
    throw new Error('Order state changed before cancellation');
  }

  await issueRefund(order.paymentId, prisma);
}

const disputeTransitions: Record<string, string[]> = {
  OPEN: ['RESOLVED', 'ESCALATED'],
  ESCALATED: ['RESOLVED'],
  RESOLVED: [],
};

export async function resolveDispute(prisma: PrismaClient, disputeId: string, resolution: string) {
  const dispute = await prisma.dispute.findUnique({
    where: { id: disputeId },
    select: { status: true },
  });

  if (!dispute || !disputeTransitions[dispute.status]?.includes('RESOLVED')) {
    throw new Error('Dispute cannot be resolved in its current state');
  }

  const updated = await prisma.dispute.updateMany({
    where: { id: disputeId, status: dispute.status },
    data: { status: 'RESOLVED', resolution },
  });

  if (updated.count === 0) throw new Error('Concurrent modification');

  await payoutSeller(disputeId, prisma);
}
