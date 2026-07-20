// SAFE: Uses UUIDs and checks ownership before returning data
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = req.params.id;
  const session = getSession(req);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(invoiceId, session.userId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function listOrders(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const orders = await db.prepare('SELECT * FROM orders WHERE user_id = ? ORDER BY created_at').bind(session.userId).all();
  return new Response(JSON.stringify(orders));
}
