// SAFE: Template literal not used — proper parameterization with ownership
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(req.params.id, session.userId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}
