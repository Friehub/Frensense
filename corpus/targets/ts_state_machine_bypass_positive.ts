// [frensense]
// observation: An entity is transitioned to a terminal state (e.g., CANCELLED, REFUNDED, DELIVERED) without checking its current logical state.
// impact: An attacker can refund an already cancelled order, or cancel a shipped order, bypassing the business logic state machine.
// improvement: Assert that the current state is valid for the transition (e.g., only allow CANCEL if state is PENDING).

async function cancelOrder(orderId: string, db: DB) {
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first();
  if (!order) throw new Error('Not found');

  // VULNERABLE: blindly updates state without asserting it was PENDING
  await db.prepare('UPDATE orders SET status = "CANCELLED" WHERE id = ?').bind(orderId).run();
  
  // triggers a refund, even if the order was already completed or cancelled!
  await issueRefund(order.payment_id);
}

const resolveDisputeProcedure = protectedProcedure.mutation(async ({ ctx, input }) => {
  // VULNERABLE: dispute can be resolved multiple times, paying out multiple times
  await prisma.dispute.update({
    where: { id: input.disputeId },
    data: { status: 'RESOLVED', resolution: input.resolution }
  });
  await payoutSeller(input.disputeId);
  return { success: true };
});
