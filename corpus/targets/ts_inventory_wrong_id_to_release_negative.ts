// SAFE: Passes the correct reservationId by looking it up from the order

export async function cancelOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  // SAFE: find the reservation from the order
  const reservation = await env.DB.prepare(
    'SELECT id FROM inventory_reservations WHERE order_id = ? AND released = 0'
  ).bind(orderId).first();

  if (reservation) {
    await releaseStock(reservation.id, env);
  }

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('CANCELLED', orderId).run();

  if (order.payment_intent) {
    await env.STRIPE.refunds.create({ payment_intent: order.payment_intent });
  }
}

async function releaseStock(reservationId: string, env: Env) {
  await env.DB.prepare(
    'UPDATE inventory_reservations SET released = 1 WHERE id = ?'
  ).bind(reservationId).run();
}
