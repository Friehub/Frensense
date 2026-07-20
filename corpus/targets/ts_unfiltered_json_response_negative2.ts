// SAFE: Uses class-transformer to strip sensitive fields via @Exclude decorator
import { Exclude, classToPlain } from "class-transformer";

class UserDTO {
  id: number;
  name: string;
  publicBio: string;

  @Exclude()
  passwordHash: string;

  @Exclude()
  internalNote: string;
}

async function getUserProfile(userId: string, db: DB) {
  const row = await db.prepare("SELECT * FROM users WHERE id = ?").bind(userId).first();
  if (!row) return Response.json({ error: "Not found" }, { status: 404 });
  const dto = Object.assign(new UserDTO(), row);
  return Response.json(classToPlain(dto));
}

const getWorkspace = protectedProcedure.query(async ({ ctx }) => {
  const workspace = await prisma.workspace.findUnique({
    where: { id: ctx.session.workspaceId }
  });
  return { workspace: { id: workspace.id, name: workspace.name, createdAt: workspace.createdAt } };
});
