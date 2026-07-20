// SAFE: Uses Prisma in a transaction to cancel subscription and delete entitlements atomically

export async function cancelSubscription(prisma: PrismaClient, subscriptionId: string) {
  return prisma.$transaction(async (tx) => {
    const sub = await tx.subscription.findUnique({
      where: { id: subscriptionId },
      include: { user: true },
    });

    if (!sub) throw new Error('Subscription not found');

    await tx.subscription.update({
      where: { id: subscriptionId },
      data: { status: 'CANCELLED', cancelledAt: new Date() },
    });

    await tx.entitlement.deleteMany({
      where: { userId: sub.userId },
    });

    if (sub.user.stripeCustomerId) {
      await stripe.subscriptions.cancel(sub.stripeSubscriptionId!);
    }
  });
}
