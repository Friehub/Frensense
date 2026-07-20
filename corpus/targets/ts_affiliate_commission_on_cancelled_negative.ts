// SAFE: Defers commission payout until the order is completed, and reverses on cancellation

export async function processOrder(userId: string, affiliateCode: string, total: number, env: Env) {
  const order = await env.DB.prepare(
    'INSERT INTO orders (user_id, total, status, affiliate_code, pending_commission) VALUES (?, ?, ?, ?, ?) RETURNING id'
  ).bind(userId, total, 'PENDING', affiliateCode, total * 0.1).first();

  return { orderId: order.id };
}

export async function completeOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ? AND status = ?'
  ).bind(orderId, 'DELIVERED').first();

  if (!order) throw new Error('Order not found or not delivered');

  if (order.affiliate_code && order.pending_commission > 0) {
    const commission = order.pending_commission;
    const affiliate = await env.DB.prepare(
      'SELECT id FROM affiliates WHERE code = ?'
    ).bind(order.affiliate_code).first();

    if (affiliate) {
      await env.DB.prepare(
        'UPDATE affiliates SET balance = balance + ? WHERE id = ?'
      ).bind(commission, affiliate.id).run();

      await env.DB.prepare(
        'INSERT INTO affiliate_transactions (affiliate_id, order_id, amount, type) VALUES (?, ?, ?, ?)'
      ).bind(affiliate.id, orderId, commission, 'commission').run();
    }
  }

  await env.DB.prepare(
    'UPDATE orders SET status = ?, pending_commission = 0 WHERE id = ?'
  ).bind('COMPLETED', orderId).run();
}

export async function cancelOrder(orderId: string, env: Env) {
  await env.DB.prepare(
    'UPDATE orders SET status = ?, pending_commission = 0 WHERE id = ?'
  ).bind('CANCELLED', orderId).run();
}
