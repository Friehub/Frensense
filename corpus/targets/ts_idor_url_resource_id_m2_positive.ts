// [frensense]
// observation: Resource ID from URL parameter flows through an intermediate variable into a database query without ownership verification.
// impact: An attacker can access other users' resources by guessing or enumerating sequential resource IDs.
// improvement: Always verify that the authenticated user owns the requested resource before returning data.

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = req.params.id;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(invoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const orderId = req.params.orderId;
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
