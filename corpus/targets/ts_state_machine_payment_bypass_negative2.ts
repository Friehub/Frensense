// SAFE: Uses Prisma's updateMany with a status filter and idempotency key to prevent replay

export async function handleStripeWebhook(prisma: PrismaClient, event: StripeEvent) {
  if (event.type === 'payment_intent.succeeded') {
    const paymentIntent = event.data.object as PaymentIntent;
    const orderId = paymentIntent.metadata.orderId;
    const idempotencyKey = `stripe:${event.id}`;

    const existing = await prisma.idempotencyKey.findUnique({
      where: { key: idempotencyKey }
    });
    if (existing) return { status: 'already_processed' };

    const updated = await prisma.order.updateMany({
      where: {
        id: orderId,
        status: { in: ['PENDING', 'AWAITING_CONFIRMATION'] }
      },
      data: { status: 'COMPLETED' }
    });

    if (updated.count === 0) {
      return { status: 'ignored', reason: 'invalid_state' };
    }

    await prisma.idempotencyKey.create({ data: { key: idempotencyKey } });
    await prisma.payment.updateMany({
      where: { stripePi: paymentIntent.id },
      data: { status: 'completed' }
    });
  }

  return { status: 'processed' };
}
