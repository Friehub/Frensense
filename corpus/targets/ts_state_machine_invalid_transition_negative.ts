// SAFE: Validates the transition against a whitelist before allowing the status change

const ALLOWED_TRANSITIONS: Record<string, string[]> = {
  PENDING: ['PROCESSING', 'CANCELLED'],
  PROCESSING: ['SHIPPED', 'CANCELLED'],
  SHIPPED: ['DELIVERED'],
  DELIVERED: ['RETURN_REQUESTED'],
  RETURN_REQUESTED: ['RETURNED'],
  COMPLETED: [],
  CANCELLED: [],
  RETURNED: [],
};

export async function adminUpdateOrderStatus(
  orderId: string,
  newStatus: string,
  env: Env
) {
  const order = await env.DB.prepare(
    'SELECT status FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  const allowed = ALLOWED_TRANSITIONS[order.status];
  if (!allowed || !allowed.includes(newStatus)) {
    throw new Error(
      `Cannot transition from ${order.status} to ${newStatus}`
    );
  }

  const result = await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ? AND status = ?'
  ).bind(newStatus, orderId, order.status).run();

  if (result.meta.changes === 0) {
    throw new Error('Concurrent state change detected');
  }

  if (newStatus === 'CANCELLED') {
    await issueRefund(orderId, env);
  }

  return { success: true };
}
