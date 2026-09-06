// SAFE: Verifies the requesting user has manager role before allowing assignment
export async function assignTask(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'manager' && session.role !== 'admin') {
    return new Response('Forbidden', { status: 403 });
  }
  const { taskId, assigneeId } = await req.json();
  await db.prepare('UPDATE tasks SET assignee_id = ?, status = ?, assigned_by = ? WHERE id = ?').bind(assigneeId, 'assigned', session.userId, taskId).run();
  return new Response(JSON.stringify({ assigned: true }));
}

export async function assignLead(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'manager') return new Response('Forbidden', { status: 403 });
  const { leadId, salespersonId } = req.body;
  await db.prepare('UPDATE leads SET owner_id = ?, updated_by = ? WHERE id = ?').bind(salespersonId, session.userId, leadId).run();
  return new Response('Assigned');
}
