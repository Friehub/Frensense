// SAFE: Per-ID ownership verified with individual queries in transaction
export async function batchGetOrders(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { ids } = await req.json();
  const results = [];
  for (const id of ids) {
    const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(id, session.userId).first();
    if (order) results.push(order);
  }
  return new Response(JSON.stringify(results));
}
