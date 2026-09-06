// SAFE: Multi-hop with UUID and ownership check
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const a = req.params.id;
  const b = a;
  const invoiceId = b;
  const session = getSession(req);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(invoiceId, session.userId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}
