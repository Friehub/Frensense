// [frensense]
// observation: Loyalty points are credited to the user's account as soon as an order is placed, before the order is fulfilled or delivered.
// impact: A user can place an order, receive points, then cancel the order, gaining free points that can be redeemed for real value.
// improvement: Defer point crediting until the order reaches a fulfilled/delivered state.

export async function placeOrder(userId: string, total: number, env: Env) {
  const order = await env.DB.prepare(
    'INSERT INTO orders (user_id, total, status) VALUES (?, ?, ?) RETURNING id'
  ).bind(userId, total, 'PENDING').first();

  // VULNERABLE: points awarded before order is fulfilled
  const pointsEarned = Math.floor(total * 0.1);
  await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points + ? WHERE id = ?'
  ).bind(pointsEarned, userId).run();

  return { orderId: order.id, pointsEarned };
}
