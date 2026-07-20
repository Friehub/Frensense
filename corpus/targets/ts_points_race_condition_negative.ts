// SAFE: Uses atomic decrement with a points guard to prevent race conditions

export async function redeemPoints(userId: string, cost: number, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points - ? WHERE id = ? AND loyalty_points >= ?'
  ).bind(cost, userId, cost).run();

  if (result.meta.changes === 0) {
    throw new Error('Insufficient points');
  }

  await env.DB.prepare(
    'INSERT INTO point_redemptions (user_id, cost) VALUES (?, ?)'
  ).bind(userId, cost).run();
}
