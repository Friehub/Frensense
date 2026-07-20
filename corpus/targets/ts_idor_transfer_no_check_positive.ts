// [frensense]
// observation: A resource ownership transfer operation accepts the target owner from the request body without verifying that the current user owns the resource.
// impact: An attacker can transfer any resource to their own account, effectively stealing resources from other users.
// improvement: Verify that the current user owns the resource before allowing transfer, and consider requiring confirmation for ownership changes.

export async function transferDocument(req: Request, db: DB): Promise<Response> {
  const { docId, newOwnerId } = await req.json();
  await db.prepare('UPDATE documents SET owner_id = ? WHERE id = ?').bind(newOwnerId, docId).run();
  return new Response(JSON.stringify({ transferred: true }));
}

export async function reassignTicket(req: Request, db: DB): Promise<Response> {
  const { ticketId, assigneeId } = req.body;
  await db.prepare('UPDATE tickets SET assignee_id = ? WHERE id = ?').bind(assigneeId, ticketId).run();
  return new Response('Reassigned');
}
