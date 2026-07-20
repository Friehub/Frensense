// SAFE: Uses atomic UPDATE with the current state in the WHERE clause, eliminating the race window

export async function fulfillOrder(orderId: string, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ? AND status = ?'
  ).bind('SHIPPED', orderId, 'PROCESSING').run();

  if (result.meta.changes === 0) {
    const order = await env.DB.prepare(
      'SELECT status FROM orders WHERE id = ?'
    ).bind(orderId).first();
    throw new Error(
      `Cannot fulfill: order is in state ${order?.status}`
    );
  }

  await generateShippingLabel(orderId, env);
}

async function completeOrder(orderId: string, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ? AND status = ?'
  ).bind('COMPLETED', orderId, 'SHIPPED').run();

  if (result.meta.changes === 0) {
    const order = await env.DB.prepare(
      'SELECT status FROM orders WHERE id = ?'
    ).bind(orderId).first();
    throw new Error(
      `Cannot complete: order is in state ${order?.status}`
    );
  }

  await releasePayout(orderId, env);
}
