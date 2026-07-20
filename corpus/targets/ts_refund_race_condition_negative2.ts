// SAFE: Uses Prisma $transaction to serialize the eligibility check and status update

export async function processRefund(prisma: PrismaClient, orderId: string) {
  await prisma.$transaction(async (tx) => {
    const order = await tx.order.findUnique({
      where: { id: orderId },
      select: { status: true, stripePi: true },
    });

    if (!order || order.status !== 'DELIVERED') {
      throw new Error('Order is not eligible for refund');
    }

    await tx.order.update({
      where: { id: orderId },
      data: { status: 'REFUND_PENDING' },
    });

    await stripe.refunds.create({
      payment_intent: order.stripePi!,
    });
  });
}
