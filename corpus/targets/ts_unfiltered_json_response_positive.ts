// [frensense]
// observation: Database query retrieves wildcard rows (SELECT *) and returns them directly in a JSON response.
// impact: Internal flags, password hashes, or PII are unintentionally exposed to the client via the API response.
// improvement: Map database results to explicit DTO objects or use SELECT with specific column names before sending.

async function getUserProfile(userId: string, db: DB) {
  // VULNERABLE: select * directly sent to JSON
  const row = await db.prepare('SELECT * FROM users WHERE id = ?').bind(userId).first();
  if (!row) return Response.json({ error: 'Not found' }, { status: 404 });
  return Response.json(row);
}

const getWorkspace = protectedProcedure.query(async ({ ctx }) => {
  // VULNERABLE: Prisma payload with all fields returned
  const workspace = await prisma.workspace.findUnique({
    where: { id: ctx.session.workspaceId }
  });
  return { workspace };
});
