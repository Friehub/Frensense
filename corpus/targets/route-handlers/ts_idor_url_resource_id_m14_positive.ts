// [frensense]
// observation: Resource ID from URL parameter is used without ownership verification with renamed variables.
// impact: An attacker can access other users' resources.
// improvement: Verify authenticated user owns the resource
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const requestedInvoiceId = req.params.id;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(requestedInvoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const requestedOrderId = req.params.orderId;
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(requestedOrderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
