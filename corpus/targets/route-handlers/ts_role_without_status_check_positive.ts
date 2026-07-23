// [frensense]
// observation: Middleware checks role membership but not the associated profile's lifecycle status.
// impact: Unverified, suspended, or rejected users can perform all role-gated operations.
// improvement: Query the profile record and assert status === 'ACTIVE' before calling next().

// VULNERABLE: role checked, status never verified
const sellerProcedure = protectedProcedure.use(({ ctx, next }) => {
  if (ctx.session.user.role !== 'SELLER' && ctx.session.user.role !== 'ADMIN') {
    throw new TRPCError({ code: 'FORBIDDEN' });
  }
  // MISSING: no check that ctx.session.user's Seller profile.status === 'ACTIVE'
  return next();
});

async function handleSellerDashboard(req: Request, session: Session, db: DB) {
  if (session.role !== 'seller') {
    return Response.json({ error: 'forbidden' }, { status: 403 });
  }
  // MISSING: session.status is never checked; suspended sellers reach here
  const metrics = await db.prepare('SELECT * FROM seller_metrics WHERE seller_id = ?')
    .bind(session.sellerId).all();
  return Response.json(metrics);
}
