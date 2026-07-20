// [frensense]
// observation: Resource ID from URL parameter is used without ownership verification inside a try-catch block.
// impact: An attacker can access other users' resources, with errors silently caught.
// improvement: Verify authenticated user owns the resource

export async function getInvoice(req: Request, db: DB): Promise<Response> {
  try {
    const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ?').bind(req.params.id).first();
    if (!invoice) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(invoice));
  } catch (err) { console.error(err); return new Response('Error', { status: 500 }); }
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  try {
    const order = await db.prepare('SELECT * FROM orders WHERE id = ?').bind(req.params.orderId).first();
    if (!order) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(order));
  } catch { return new Response('Error', { status: 500 }); }
}
