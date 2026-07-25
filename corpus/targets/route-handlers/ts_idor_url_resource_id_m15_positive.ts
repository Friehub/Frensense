// [frensense]
// observation: Resource ID from URL parameter is used without ownership verification via a promise .then() chain.
// impact: An attacker can access other users' resources.
// improvement: Verify authenticated user owns the resource
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

export function getInvoice(req: Request, db: DB): Promise<Response> {
  return Promise.resolve(req.params.id).then(id => {
    return db.prepare('SELECT * FROM invoices WHERE id = ?').bind(id).first().then(invoice => {
      if (!invoice) return new Response('Not found', { status: 404 });
      return new Response(JSON.stringify(invoice));
    });
  });
}

export function getOrder(req: Request, db: DB): Promise<Response> {
  return Promise.resolve(req.params.orderId).then(orderId => {
    return db.prepare('SELECT * FROM orders WHERE id = ?').bind(orderId).first().then(order => {
      if (!order) return new Response('Not found', { status: 404 });
      return new Response(JSON.stringify(order));
    });
  });
}
