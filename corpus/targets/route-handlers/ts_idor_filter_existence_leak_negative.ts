// SAFE: Resources are always scoped to the authenticated user before returning
export async function searchInvoices(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { invoiceId } = req.query;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(invoiceId, session.userId).first();
  if (!invoice) return new Response(JSON.stringify({ results: [] }));
  return new Response(JSON.stringify(invoice));
}

export async function checkOrderStatus(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(req.params.orderId, session.userId).first();
  if (!order) return new Response(JSON.stringify({ results: [] }));
  return new Response(JSON.stringify({ status: order.status }));
}
