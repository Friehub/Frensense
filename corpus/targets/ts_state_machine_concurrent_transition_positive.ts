// [frensense]
// observation: Two concurrent requests both read the order state, find it valid for transition, and both proceed to update — a classic TOCTOU race condition in state transitions.
// impact: An attacker can double-claim a reward or trigger two shipments for a single order by sending simultaneous transition requests.
// improvement: Use an atomic UPDATE with the current state in the WHERE clause, or use database row-level locking inside a transaction.

export async function fulfillOrder(orderId: string, env: Env) {
  // VULNERABLE: read, then check, then write — race window between them
  const order = await env.DB.prepare(
    'SELECT status FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order || order.status !== 'PROCESSING') {
    throw new Error('Order cannot be fulfilled');
  }

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('SHIPPED', orderId).run();

  await generateShippingLabel(orderId, env);
}

async function completeOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT status FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order || order.status !== 'SHIPPED') {
    throw new Error('Order cannot be completed');
  }

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('COMPLETED', orderId).run();

  await releasePayout(orderId, env);
}
