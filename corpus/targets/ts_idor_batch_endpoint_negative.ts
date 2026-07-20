// SAFE: Ownership verified for each resource ID in the batch
export async function batchGetOrders(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { ids } = await req.json();
  const placeholders = ids.map(() => '?').join(',');
  const orders = await db.prepare(`SELECT * FROM orders WHERE id IN (${placeholders}) AND user_id = ?`).bind(...ids, session.userId).all();
  return new Response(JSON.stringify(orders));
}

export async function batchDeleteMessages(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const { messageIds } = await req.json();
  const placeholders = messageIds.map(() => '?').join(',');
  await db.prepare(`DELETE FROM messages WHERE id IN (${placeholders}) AND user_id = ?`).bind(...messageIds, session.userId).run();
  return new Response(JSON.stringify({ deleted: messageIds.length }));
}
