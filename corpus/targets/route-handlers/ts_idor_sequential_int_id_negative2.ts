// SAFE: Uses UUID primary keys making IDs unguessable, plus ownership check
import { v4 as uuidv4 } from 'uuid';

export async function createOrder(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const id = uuidv4();
  await db.prepare('INSERT INTO orders (id, user_id, total, status) VALUES (?, ?, ?, ?)').bind(id, session.userId, req.body.total, 'pending').run();
  return new Response(JSON.stringify({ id }), { status: 201 });
}

export async function getOrder(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const order = await db.prepare('SELECT * FROM orders WHERE id = ? AND user_id = ?').bind(req.params.id, session.userId).first();
  if (!order) return new Response('Not found', { status: 404 });
  return new Response(JSON.stringify(order));
}
