// [frensense]
// observation: A batch endpoint processes multiple resource IDs in a single request without verifying ownership for each ID.
// impact: An attacker can include resource IDs belonging to other users in the batch request and learn or modify data they should not have access to, bypassing per-ID access controls.
// improvement: Verify ownership for every resource ID in the batch before processing.

export async function batchGetOrders(req: Request, db: DB): Promise<Response> {
  const { ids } = await req.json();
  const placeholders = ids.map(() => '?').join(',');
  const orders = await db.prepare(`SELECT * FROM orders WHERE id IN (${placeholders})`).bind(...ids).all();
  return new Response(JSON.stringify(orders));
}

export async function batchDeleteMessages(req: Request, db: DB): Promise<Response> {
  const { messageIds } = await req.json();
  const placeholders = messageIds.map(() => '?').join(',');
  await db.prepare(`DELETE FROM messages WHERE id IN (${placeholders})`).bind(...messageIds).run();
  return new Response(JSON.stringify({ deleted: messageIds.length }));
}
