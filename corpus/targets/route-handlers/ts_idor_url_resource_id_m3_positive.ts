// [frensense]
// observation: Resource ID from URL parameter traverses multiple variable assignments before being used in a query without ownership verification.
// impact: An attacker can access other users' resources through multi-hop ID assignment by guessing sequential IDs.
// improvement: Always verify ownership regardless of how many assignment hops occur.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const a = req.params.id;
  const b = a;
  const invoiceId = b;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(invoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const raw = req.params.orderId;
  const orderId = raw;
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
