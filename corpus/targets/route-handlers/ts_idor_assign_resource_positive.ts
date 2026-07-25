// [frensense]
// observation: An endpoint that assigns resources to users does not verify that the requesting user has the required role (e.g., manager, admin) to perform the assignment.
// impact: Any authenticated user can assign resources (e.g., tasks, leads, tickets) to any other user, enabling unauthorized team management and data access.
// improvement: Check that the requesting user has the required role (manager, admin) before allowing resource assignment.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function assignTask(req: Request, db: DB): Promise<Response> {
  const { taskId, assigneeId } = await req.json();
  await db.prepare('UPDATE tasks SET assignee_id = ?, status = ? WHERE id = ?').bind(assigneeId, 'assigned', taskId).run();
  return new Response(JSON.stringify({ assigned: true }));
}

export async function assignLead(req: Request, db: DB): Promise<Response> {
  const { leadId, salespersonId } = req.body;
  await db.prepare('UPDATE leads SET owner_id = ? WHERE id = ?').bind(salespersonId, leadId).run();
  return new Response('Assigned');
}
