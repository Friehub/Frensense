// [frensense]
// observation: Resource ID from URL parameter is used without ownership verification inside a conditional block on the tainted branch.
// impact: An attacker can access other users' resources.
// improvement: Verify authenticated user owns the resource
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  if (req.params.id) {
    const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(req.params.id).first();
    if (!invoice) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(invoice));
  }
  return new Response('Missing id', { status: 400 });
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  if (req.params.orderId) {
    const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(req.params.orderId).first();
    if (!order) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(order));
  }
  return new Response('Missing orderId', { status: 400 });
}
