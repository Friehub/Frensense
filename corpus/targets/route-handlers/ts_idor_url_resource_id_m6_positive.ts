// [frensense]
// observation: Resource ID from URL parameter is concatenated into a query string without ownership verification.
// impact: An attacker can access other users' resources by supplying any ID through string concatenation in the query.
// improvement: Use parameterized queries and add an ownership check on the user_id column.

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = \'' + req.params.id + '\'').first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const order = await db.prepare('SELECT * FROM orders WHERE id = \'' + req.params.orderId + '\'').first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
