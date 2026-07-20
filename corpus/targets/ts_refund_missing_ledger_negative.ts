// SAFE: Every wallet credit is paired with a corresponding ledger entry to maintain double-entry integrity

export async function issueRefund(userId: string, orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT total, stripe_pi FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // SAFE: create ledger entry alongside the wallet credit
  const ledgerId = crypto.randomUUID();
  await env.DB.prepare(
    'INSERT INTO ledger (id, user_id, type, amount, reference_type, reference_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)'
  ).bind(ledgerId, userId, 'REFUND', Number(order.total), 'order', orderId, Date.now()).run();

  await env.DB.prepare(
    'UPDATE wallets SET balance = balance + ? WHERE user_id = ?'
  ).bind(Number(order.total), userId).run();

  await env.DB.prepare(
    'UPDATE orders SET status = ?, ledger_id = ? WHERE id = ?'
  ).bind('REFUNDED', ledgerId, orderId).run();
}
