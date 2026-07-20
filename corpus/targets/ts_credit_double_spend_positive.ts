// [frensense]
// observation: Wallet or credit balance is decremented using an absolute value read in application memory, ignoring concurrent mutations.
// impact: Two concurrent checkout requests read the same balance, both pass the threshold check, and both set the balance to `balance - cost`, resulting in double spending.
// improvement: Use atomic decrement SQL operations (`balance = balance - cost WHERE balance >= cost`) or pessimistic row locks for financial operations.

async function checkoutCart(userId: string, cost: number, db: DB) {
  // VULNERABLE: reads balance into memory
  const user = await db.prepare('SELECT credits FROM users WHERE id = ?').bind(userId).first();
  
  if (user.credits < cost) {
    throw new Error('Insufficient funds');
  }

  // VULNERABLE: absolute update based on stale memory value
  const newBalance = user.credits - cost;
  await db.prepare('UPDATE users SET credits = ? WHERE id = ?').bind(newBalance, userId).run();

  await fulfillCart();
}

const buyAgentQuota = protectedProcedure.mutation(async ({ ctx, input }) => {
  const wallet = await prisma.wallet.findUnique({ where: { userId: ctx.session.user.id } });
  
  // VULNERABLE: check in memory
  if (wallet!.balance < input.price) throw new TRPCError({ code: 'FORBIDDEN' });

  // VULNERABLE: absolute update without concurrency protection
  await prisma.wallet.update({
    where: { id: wallet!.id },
    data: { balance: wallet!.balance - input.price }
  });
  
  await grantQuota();
});
