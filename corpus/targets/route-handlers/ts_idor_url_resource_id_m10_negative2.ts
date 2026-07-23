// SAFE: Async path with ownership check via JOIN
async function getInvoiceId(req: any): Promise<string> { return req.params.id; }
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = await getInvoiceId(req);
  const invoice = await db.prepare('SELECT i.* FROM invoices i JOIN users u ON i.user_id = u.id WHERE i.id = ? AND u.id = ?').bind(invoiceId, req.user.id).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}
