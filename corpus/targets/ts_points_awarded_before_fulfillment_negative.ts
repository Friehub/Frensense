// SAFE: Defers point crediting until the order is fulfilled/delivered

export async function placeOrder(userId: string, total: number, env: Env) {
  const order = await env.DB.prepare(
    'INSERT INTO orders (user_id, total, status, pending_points) VALUES (?, ?, ?, ?) RETURNING id'
  ).bind(userId, total, 'PENDING', Math.floor(total * 0.1)).first();

  return { orderId: order.id };
}

export async function fulfillOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ? AND status = ?'
  ).bind(orderId, 'PROCESSING').first();

  if (!order) throw new Error('Order not found');

  // SAFE: award points only on fulfillment
  await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points + ? WHERE id = ?'
  ).bind(order.pending_points, order.user_id).run();

  await env.DB.prepare(
    'UPDATE orders SET status = ?, pending_points = 0 WHERE id = ?'
  ).bind('FULFILLED', orderId).run();
}
