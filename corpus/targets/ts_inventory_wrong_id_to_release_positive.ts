// [frensense]
// observation: The stock release function receives an orderId instead of a reservationId, attempting to release the wrong record and failing to restore inventory.
// impact: Cancelling orders never actually releases stock because the releaseStock function queries by the wrong identifier, causing indefinite stock locking.
// improvement: Pass the correct reservationId (not orderId) to the releaseStock function, or join through the order to find the reservation.

export async function cancelOrder(orderId: string, env: Env) {
  const order = await env.DB.prepare(
    'SELECT * FROM orders WHERE id = ?'
  ).bind(orderId).first();

  if (!order) throw new Error('Order not found');

  await env.DB.prepare(
    'UPDATE orders SET status = ? WHERE id = ?'
  ).bind('CANCELLED', orderId).run();

  // VULNERABLE: passes orderId instead of reservationId
  await releaseStock(orderId, env);

  if (order.payment_intent) {
    await env.STRIPE.refunds.create({ payment_intent: order.payment_intent });
  }
}

async function releaseStock(reservationId: string, env: Env) {
  await env.DB.prepare(
    'UPDATE inventory_reservations SET released = 1 WHERE id = ?'
  ).bind(reservationId).run();
}
