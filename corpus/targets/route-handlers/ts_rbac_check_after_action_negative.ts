// SAFE: Authorization check is performed before any sensitive operation
export async function processRefund(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin' && session.role !== 'support') {
    return new Response('Forbidden', { status: 403 });
  }
  const { orderId, amount } = await req.json();
  const refund = await db.prepare('INSERT INTO refunds (order_id, amount, status) VALUES (?, ?, ?)').bind(orderId, amount, 'pending').run();
  return new Response(JSON.stringify({ refundId: refund.id }));
}

export async function deleteUser(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  if (session.role !== 'admin') return new Response('Forbidden', { status: 403 });
  const userId = req.params.id;
  await db.prepare('DELETE FROM users WHERE id = ?').bind(userId).run();
  return new Response('Deleted');
}
