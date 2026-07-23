// SAFE: Async path with ownership verification
async function getInvoiceId(req: any): Promise<string> { return req.params.id; }
async function getOrderId2(req: any): Promise<string> { return req.params.orderId; }

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = await getInvoiceId(req);
  const userId = req.user.id;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(invoiceId, userId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const orderId = await getOrderId2(req);
  const userId = req.user.id;
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(orderId, userId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
