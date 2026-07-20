// SAFE: Only allows modification if the order is in a modifiable state

const MODIFIABLE_STATES = ['PENDING', 'PROCESSING'];

export async function modifyOrder(orderId: string, updates: Record<string, any>, env: Env) {
  const order = await env.DB.prepare(
    'SELECT status FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  if (!MODIFIABLE_STATES.includes(order.status)) {
    throw new Error(`Order cannot be modified in ${order.status} state`);
  }

  await env.DB.prepare(
    'UPDATE orders SET ? WHERE id = ? AND status = ?'
  ).bind(updates, orderId, order.status).run();

  return { modified: true };
}
