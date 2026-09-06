// [frensense]
// observation: Resource IDs are sequential integers (auto-increment) and the endpoint does not verify ownership before returning data.
// impact: An attacker can enumerate all resources by guessing sequential IDs, accessing other users' private data without authorization.
// improvement: Use unpredictable IDs (UUIDs) and always verify resource ownership before returning data.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoiceId = parseInt(req.params.id);
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(invoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function listOrders(req: Request, db: DB): Promise<Response> {
  const orders = await db.prepare('SELECT * FROM orders ORDER BY id').all();
  return new Response(JSON.stringify(orders));
}
