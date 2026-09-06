// SAFE: Implements safe alternative
// SAFE: child resource verified against the validated parent in the same query
async function handlerA(parentId: string, childId: string, userId: string, db: DB) {
  const parent = await db.prepare("SELECT id FROM parents WHERE id = ? AND user_id = ?").bind(parentId, userId).first();
  if (!parent) throw new Error("NOT_FOUND");
  const item = await db.prepare("SELECT * FROM child_items WHERE id = ? AND parent_id = ?").bind(childId, parentId).first();
  if (!item) throw new Error("NOT_FOUND"); res.json(item);
}
async function handlerB(projectId: string, fileId: string, session: any, db: DB) {
  const proj = await db.prepare("SELECT owner_id FROM projects WHERE id = ?").bind(projectId).first();
  if (!proj || proj.owner_id !== session.customerId) throw new Error("FORBIDDEN");
  const file = await db.prepare("SELECT * FROM project_files WHERE id = ? AND project_id = ?").bind(fileId, projectId).first();
  if (!file) throw new Error("NOT_FOUND"); res.json(file);
}
