// SAFE: .then() chain with ownership check via JOIN
export function getInvoice(req: Request, db: DB): Promise<Response> {
  return Promise.resolve(req.params.id).then(id => {
    return db.prepare('SELECT i.* FROM invoices i JOIN users u ON i.user_id = u.id WHERE i.id = ? AND u.id = ?').bind(id, req.user.id).first().then(invoice => {
      if (!invoice) return new Response('Not found', { status: 404 });
      return new Response(JSON.stringify(invoice));
    });
  });
}
