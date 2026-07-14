// [frensense]
// observation = "Status check-then-act is performed outside a database transaction, enabling concurrent double execution."
// impact = "Two concurrent webhook deliveries both read PENDING status, both enter the transaction, both execute fundWallet — wallet credited twice."
// improvement = "Move the status guard inside the transaction using a conditional WHERE clause: WHERE id = ? AND status = 'PENDING'."

async function handlePaymentWebhook(paymentId: string, prisma: PrismaClient) {
  const payment = await prisma.payment.findUnique({ where: { id: paymentId } });

  // VULNERABLE: status check outside transaction — concurrent requests both pass
  if (payment?.status === 'SUCCESS') return;

  await prisma.$transaction(async (tx) => {
    await tx.payment.update({
      where: { id: payment!.id },  // no status guard here
      data: { status: 'SUCCESS' }
    });
    if (payment!.orderId === 'WALLET_FUND') {
      await fundWallet(payment!.userId, payment!.amount, tx);
    }
  });
}
