// SAFE: Enforces recent authentication or explicit 2FA validation
async function changeAccountEmail(req: Request, session: Session, db: DB) {
  const newEmail = req.body.email;
  const otp = req.body.otp;

  // SAFE: verifies a one-time password before proceeding
  const isValid = await verifyTotp(session.userId, otp);
  if (!isValid) return new Response('MFA Required', { status: 403 });

  await db.prepare('UPDATE users SET email = ? WHERE id = ?')
    .bind(newEmail, session.userId).run();
  return Response.json({ success: true });
}

const updatePasswordProcedure = protectedProcedure.mutation(async ({ ctx, input }) => {
  // SAFE: verifies the session was created recently (sudo mode)
  if (Date.now() - ctx.session.authenticatedAt > 15 * 60 * 1000) {
    throw new TRPCError({ code: 'UNAUTHORIZED', message: 'Please log in again' });
  }

  await prisma.user.update({
    where: { id: ctx.session.user.id },
    data: { passwordHash: hash(input.newPassword) }
  });
  return { success: true };
});
