// [frensense]
// observation: Resource ID from URL parameter is used in a database query without ownership verification through an object property.
// impact: An attacker can access other users' resources by guessing or enumerating resource IDs.
// improvement: Always verify that the authenticated user owns the requested resource before returning data
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const params = { id: req.params.id };
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(params.id).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const args = { orderId: req.params.orderId };
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(args.orderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
