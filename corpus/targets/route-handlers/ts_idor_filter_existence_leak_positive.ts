// [frensense]
// observation: An endpoint that filters resources behaves differently when a non-owned resource exists vs when it doesn't (e.g., returns a different error or status code).
// impact: An attacker can use the filter endpoint to check for the existence of specific resources owned by other users, enabling data enumeration.
// improvement: Only return resources that the user owns. When no resources match the filter, return an empty result set rather than an error.

export async function searchInvoices(req: Request, db: DB): Promise<Response> {
  const { invoiceId } = req.query;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(invoiceId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function checkOrderStatus(req: Request, db: DB): Promise<Response> {
  const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(req.params.orderId).first();
  if (!order) return new Response(JSON.stringify({ exists: false }));
  return new Response(JSON.stringify({ exists: true, status: order.status }));
}
