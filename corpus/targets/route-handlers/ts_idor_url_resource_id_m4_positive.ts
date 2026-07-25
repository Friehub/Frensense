// [frensense]
// observation: Resource ID is extracted via a helper function from the request and used in a query without ownership verification.
// impact: An attacker can access other users' resources by controlling the URL parameter through an unsafe helper.
// improvement: Verify ownership inside or after calling the helper function.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

function extractId(req: Request): string {
    return req.params.id;
}

function extractOrderId(req: Request): string {
    return req.params.orderId;
}

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = extractId(req);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(invoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const orderId = extractOrderId(req);
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
