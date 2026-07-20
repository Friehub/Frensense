// SAFE: Confirms the reservation when payment succeeds, converting reserved stock to actual deduction

export async function confirmPayment(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // SAFE: confirm reservation — convert reserved to actual stock deduction
  await env.DB.prepare(
    'UPDATE products SET reserved = reserved - ? WHERE id = ? AND reserved >= ?'
  ).bind(order.quantity, order.product_id, order.quantity).run();

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('CONFIRMED', orderId).run();
}

export async function placeOrder(userId: string, productId: string, quantity: number, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE products SET reserved = reserved + ? WHERE id = ? AND (stock - reserved) >= ?'
  ).bind(quantity, productId, quantity).run();

  if (result.meta.changes === 0) {
    throw new Error('Insufficient stock');
  }

  await env.DB.prepare(
    'INSERT INTO orders (user_id, product_id, quantity, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, productId, quantity, 'AWAITING_PAYMENT').run();
}
