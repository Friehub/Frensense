// SAFE: Validates current state before transitioning
async function cancelOrder(orderId: string, db: DB) {
  const order = await db.prepare('SELECT status, payment_id FROM orders WHERE id = ?').bind(orderId).first();
  if (!order) throw new Error('Not found');

  // SAFE: explicitly asserts that the state is valid for cancellation
  if (order.status !== 'PENDING' && order.status !== 'PROCESSING') {
    throw new Error('Order cannot be cancelled in its current state');
  }

  // Atomic state transition
  const result = await db.prepare('UPDATE orders SET status = "CANCELLED" WHERE id = ? AND (status = "PENDING" OR status = "PROCESSING")').bind(orderId).run();
  
  if (result.meta.changes > 0) {
    await issueRefund(order.payment_id);
  }
}

const resolveDisputeProcedure = protectedProcedure.mutation(async ({ ctx, input }) => {
  // SAFE: verifies dispute is open before resolving
  const dispute = await prisma.dispute.findUnique({ where: { id: input.disputeId } });
  if (dispute?.status !== 'OPEN') throw new Error('Dispute is not open');

  const updated = await prisma.dispute.updateMany({
    where: { id: input.disputeId, status: 'OPEN' },
    data: { status: 'RESOLVED', resolution: input.resolution }
  });

  if (updated.count > 0) {
    await payoutSeller(input.disputeId);
  }
  return { success: true };
});
