// SAFE: Helper function validates ownership
function getAuthenticatedResource<T>(db: DB, table: string, id: string, userId: string): Promise<T | null> {
  return db.prepare(`SELECT * FROM ${table} WHERE id = ? AND user_id = ?`).bind(id, userId).first() as Promise<T | null>;
}

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const invoice = await getAuthenticatedResource(db, 'invoices', req.params.id, session.userId);
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}
