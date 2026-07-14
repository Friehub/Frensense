// SAFE: Map to DTO or use explicit column selection
async function getUserProfile(userId: string, db: DB) {
  // SAFE: explicitly querying only public fields
  const row = await db.prepare('SELECT id, name, public_bio FROM users WHERE id = ?').bind(userId).first();
  if (!row) return Response.json({ error: 'Not found' }, { status: 404 });
  return Response.json(row);
}

const getWorkspace = protectedProcedure.query(async ({ ctx }) => {
  const workspace = await prisma.workspace.findUnique({
    where: { id: ctx.session.workspaceId }
  });
  // SAFE: Mapping object explicitly to a Data Transfer Object
  return {
    workspace: {
      id: workspace.id,
      name: workspace.name,
      createdAt: workspace.createdAt
    }
  };
});

async function getTeamMembers(teamId: string, db: DB) {
  const members = await db.prepare('SELECT * FROM users WHERE team_id = ?').bind(teamId).all();
  // SAFE: array mapping to explicit DTO
  return Response.json(members.map(m => ({ id: m.id, role: m.role })));
}
