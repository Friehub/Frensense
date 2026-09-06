// SAFE: Conditional branch with ownership verification
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  if (req.params.id) {
    const userId = req.user.id;
    const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(req.params.id, userId).first();
    if (!invoice) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(invoice));
  }
  return new Response('Missing id', { status: 400 });
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  if (req.params.orderId) {
    const userId = req.user.id;
    const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(req.params.orderId, userId).first();
    if (!order) return new Response('Not found', { status: 404 });
    return new Response(JSON.stringify(order));
  }
  return new Response('Missing orderId', { status: 400 });
}
