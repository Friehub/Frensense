// SAFE: Object property value with ownership check
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const params = { id: req.params.id };
  const userId = req.user.id;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(params.id, userId).first();
  if (!invoice) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const args = { orderId: req.params.orderId };
  const userId = req.user.id;
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(args.orderId, userId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
