// SAFE: Uses atomic decrement in the database layer
async function checkoutCart(userId: string, cost: number, db: DB) {
  // SAFE: atomic UPDATE ensures balance never drops below zero, even under concurrency
  const result = await db.prepare('UPDATE users SET credits = credits - ? WHERE id = ? AND credits >= ?').bind(cost, userId, cost).run();
  
  if (result.meta.changes === 0) {
    throw new Error('Insufficient funds or user not found');
  }

  await fulfillCart();
}

const buyAgentQuota = protectedProcedure.mutation(async ({ ctx, input }) => {
  // SAFE: atomic decrement using Prisma's decrement operator
  const wallet = await prisma.wallet.updateMany({
    where: { 
      userId: ctx.session.user.id,
      balance: { gte: input.price } // ensures no negative balance
    },
    data: { 
      balance: { decrement: input.price } 
    }
  });

  if (wallet.count === 0) throw new TRPCError({ code: 'FORBIDDEN', message: 'Insufficient funds' });
  
  await grantQuota();
});
