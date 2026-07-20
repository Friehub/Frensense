// SAFE: .then() chain with ownership verification
export function getInvoice(req: Request, db: DB): Promise<Response> {
  return Promise.resolve(req.params.id).then(id => {
    const userId = req.user.id;
    return db.prepare('SELECT * FROM invoices WHERE id = ? AND user_id = ?').bind(id, userId).first().then(invoice => {
      if (!invoice) return new Response('Not found', { status: 404 });
      return new Response(JSON.stringify(invoice));
    });
  });
}

export function getOrder(req: Request, db: DB): Promise<Response> {
  return Promise.resolve(req.params.orderId).then(orderId => {
    const userId = req.user.id;
    return db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(orderId, userId).first().then(order => {
      if (!order) return new Response('Not found', { status: 404 });
      return new Response(JSON.stringify(order));
    });
  });
}
