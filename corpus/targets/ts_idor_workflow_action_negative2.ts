// SAFE: Workflow action verifies resource ownership and role in a single query
export async function approveDocument(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { documentId } = req.body;
  const result = await db.prepare(`
    UPDATE documents SET status = 'approved', approved_by = ?, approved_at = ?
    WHERE id = ? AND owner_id = ? AND (
      SELECT role FROM users WHERE id = ? ) IN ('manager', 'admin')
  `).bind(session.userId, Date.now(), documentId, session.userId, session.userId).run();
  if (!result) return new Response('Forbidden', { status: 403 });
  return new Response(JSON.stringify({ approved: true }));
}

export async function rejectExpenseReport(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { reportId, reason } = req.body;
  const report = await db.prepare('SELECT * FROM expense_reports WHERE id = ? AND owner_id = ?').bind(reportId, session.userId).first();
  if (!report || !['manager', 'admin'].includes(session.role)) return new Response('Forbidden', { status: 403 });
  await db.prepare('UPDATE expense_reports SET status = ?, rejection_reason = ?, reviewed_by = ? WHERE id = ?').bind('rejected', reason, session.userId, reportId).run();
  return new Response(JSON.stringify({ rejected: true }));
}
