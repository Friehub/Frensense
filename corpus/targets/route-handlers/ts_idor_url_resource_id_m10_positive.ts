// [frensense]
// observation: Resource ID from URL parameter is used without ownership verification across an async/await boundary.
// impact: An attacker can access other users' resources.
// improvement: Always verify that the authenticated user owns the requested resource
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

async function getInvoiceId(req: any): Promise<string> { return req.params.id; }
async function getOrderId2(req: any): Promise<string> { return req.params.orderId; }

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = await getInvoiceId(req);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(invoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const orderId = await getOrderId2(req);
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
