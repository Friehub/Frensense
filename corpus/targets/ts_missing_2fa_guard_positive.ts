// [frensense]
// observation: A high-privileged action (e.g., password reset, email change) is executed without verifying the user's 2FA status or recent authentication.
// impact: An attacker who compromises a session token can immediately execute account-takeover actions.
// improvement: Require recent authentication or a 2FA token (e.g., assert session.recentAuth or require an OTP) for sensitive actions.

async function changeAccountEmail(req: Request, session: Session, db: DB) {
  const newEmail = req.body.email;
  // VULNERABLE: action relies solely on the session token
  await db.prepare('UPDATE users SET email = ? WHERE id = ?')
    .bind(newEmail, session.userId).run();
  return Response.json({ success: true });
}

const updatePasswordProcedure = protectedProcedure.mutation(async ({ ctx, input }) => {
  // VULNERABLE: missing re-authentication check
  await prisma.user.update({
    where: { id: ctx.session.user.id },
    data: { passwordHash: hash(input.newPassword) }
  });
  return { success: true };
});
