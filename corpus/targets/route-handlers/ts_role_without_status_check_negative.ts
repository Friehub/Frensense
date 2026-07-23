// SAFE: both role and profile status verified before granting access
const sellerProcedure = protectedProcedure.use(async ({ ctx, next }) => {
  const seller = await prisma.seller.findUnique({
    where: { userId: ctx.session.user.id },
    select: { status: true }
  });
  if (!seller || (seller.status !== 'ACTIVE' && ctx.session.user.role !== 'ADMIN')) {
    throw new TRPCError({ code: 'FORBIDDEN', message: 'Seller profile is not active' });
  }
  return next();
});

async function handleSellerDashboard(req: Request, session: Session, db: DB) {
  const sellerProfile = await db.prepare('SELECT status FROM sellers WHERE user_id = ?')
    .bind(session.userId).first();
  if (!sellerProfile || sellerProfile.status !== 'ACTIVE') {
    return Response.json({ error: 'forbidden' }, { status: 403 });
  }
  const metrics = await db.prepare('SELECT * FROM seller_metrics WHERE seller_id = ?')
    .bind(session.sellerId).all();
  return Response.json(metrics);
}
