// SAFE: Uses a database transaction with SELECT FOR UPDATE to prevent concurrent double-spend
async function checkoutCart(userId: string, cost: number, db: DB) {
  await db.prepare("BEGIN IMMEDIATE").run();
  try {
    const user = await db.prepare("SELECT credits FROM users WHERE id = ?").bind(userId).first();
    if (user.credits < cost) throw new Error('Insufficient funds');
    await db.prepare("UPDATE users SET credits = ? WHERE id = ?").bind(user.credits - cost, userId).run();
    await db.prepare("COMMIT").run();
  } catch (e) {
    await db.prepare("ROLLBACK").run();
    throw e;
  }
  await fulfillCart();
}

const buyAgentQuota = protectedProcedure.mutation(async ({ ctx, input }) => {
  return await prisma.$transaction(async (tx) => {
    const wallet = await tx.wallet.findUnique({ where: { userId: ctx.session.user.id } });
    if (!wallet || wallet.balance < input.price) throw new TRPCError({ code: 'FORBIDDEN' });
    return tx.wallet.update({
      where: { id: wallet.id },
      data: { balance: { decrement: input.price } }
    });
  });
  await grantQuota();
});
