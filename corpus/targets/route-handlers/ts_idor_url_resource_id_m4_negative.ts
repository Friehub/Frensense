// SAFE: Helper extracts ID, query includes ownership check
function extractId(req: Request): string {
    return req.params.id;
}

function extractOrderId(req: Request): string {
    return req.params.orderId;
}

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = extractId(req);
  const session = getSession(req);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(invoiceId, session.userId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const orderId = extractOrderId(req);
  const session = getSession(req);
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(orderId, session.userId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
