// SAFE: Assignment requires manager role AND the resource must be in the manager's team scope
export async function assignTask(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { taskId, assigneeId } = await req.json();
  const task = await db.prepare(`
    UPDATE tasks SET assignee_id = ?, status = 'assigned', assigned_by = ?
    WHERE id = ? AND project_id IN (SELECT project_id FROM team_members WHERE user_id = ? AND role = 'manager')
  `).bind(assigneeId, session.userId, taskId, session.userId).run();
  if (!task) return new Response('Forbidden', { status: 403 });
  return new Response(JSON.stringify({ assigned: true }));
}
