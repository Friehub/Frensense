// [frensense]
// observation: An inventory reservation is created when the order is placed, but it is never confirmed after payment succeeds, leaving the reservation in a permanent limbo state.
// impact: Reserved inventory is never converted to a confirmed deduction, so the stock remains artificially locked even after payment, potentially blocking other customers.
// improvement: After payment succeeds, confirm the reservation by either converting it to a stock deduction or releasing it.

export async function placeOrder(userId: string, productId: string, quantity: number, env: Env) {
  // Reserve inventory
  const result = await env.DB.prepare(
    'UPDATE products SET reserved = reserved + ? WHERE id = ? AND (stock - reserved) >= ?'
  ).bind(quantity, productId, quantity).run();

  if (result.meta.changes === 0) {
    throw new Error('Insufficient stock');
  }

  await env.DB.prepare(
    'INSERT INTO orders (user_id, product_id, quantity, status) VALUES (?, ?, ?, ?)'
  ).bind(userId, productId, quantity, 'AWAITING_PAYMENT').run();

  // VULNERABLE: reservation is never confirmed when payment arrives
  // The stock remains in reserved state even after payment succeeds
}
