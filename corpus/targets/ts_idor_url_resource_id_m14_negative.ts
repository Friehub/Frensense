// SAFE: Renamed variables with ownership check
export async function getInvoice(req: Request, db: DB): Promise<Response> {
  const requestedInvoiceId = req.params.id;
  const userId = req.user.id;
  const invoice = await db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(requestedInvoiceId, userId).first();
  if (!invoice) return new Response('Not found', { status: 404 }); return new Response(JSON.stringify(invoice));
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const requestedOrderId = req.params.orderId;
  const userId = req.user.id;
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(requestedOrderId, userId).first();
  if (!order) return new Response('Not found', { status: 404 }); return new Response(JSON.stringify(order));
}
