// SAFE: Ownership verified through subquery joining parent
export async function handlerA(req: Request, db: DB) {
  const session = getSession(req);
  const { parentId, childId } = req.params;
  const result = await db.prepare("SELECT c.* FROM child_items c JOIN parents p ON c.parent_id = p.id WHERE c.id = ? AND p.user_id = ?").bind(childId, session.userId).first();
  if (!result) return new Response("Not found", { status: 404 });
  return new Response(JSON.stringify(result));
}
export async function handlerB(req: Request, db: DB) {
  const session = getSession(req);
  const { projectId, fileId } = req.params;
  const result = await db.prepare("SELECT f.* FROM project_files f JOIN projects p ON f.project_id = p.id WHERE f.id = ? AND p.owner_id = ?").bind(fileId, session.userId).first();
  if (!result) return new Response("Not found", { status: 404 });
  return new Response(JSON.stringify(result));
}
