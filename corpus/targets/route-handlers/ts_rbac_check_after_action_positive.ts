// [frensense]
// observation: An RBAC authorization check occurs after the sensitive operation has already been performed, typically due to ordering in middleware or handler code.
// impact: Even if the check fails, the sensitive operation (e.g., writing to DB, sending email, charging a card) has already been executed, leading to data modification without authorization.
// improvement: Perform authorization checks before any sensitive operations. Use early-return patterns.
// cwe: CWE-284
// cvss: 8.8
// owasp: A01:2021
// severity: High

export async function processRefund(req: Request, db: DB): Promise<Response> {
  const { orderId, amount } = await req.json();
  const refund = await db.prepare('INSERT INTO refunds (order_id, amount, status) VALUES (?, ?, ?)').bind(orderId, amount, 'pending').run();
  const session = getSession(req);
  if (session.role !== 'admin' && session.role !== 'support') {
    return new Response('Forbidden', { status: 403 });
  }
  return new Response(JSON.stringify({ refundId: refund.id }));
}

export async function deleteUser(req: Request, db: DB): Promise<Response> {
  const userId = req.params.id;
  await db.prepare('DELETE FROM users WHERE id = ?').bind(userId).run();
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  return new Response('Deleted');
}
