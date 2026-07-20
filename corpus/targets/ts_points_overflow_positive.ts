// [frensense]
// observation: The loyalty points balance stored as a standard JavaScript number can overflow when users accumulate very large point totals.
// impact: A high-volume user's points balance can overflow past Number.MAX_SAFE_INTEGER, silently wrapping to a negative or incorrect value and corrupting the reward system.
// improvement: Store points as a BigInt column in the database or use a DECIMAL type with sufficient precision.

export async function awardPoints(userId: string, points: number, env: Env) {
  // VULNERABLE: no overflow check for large point balances
  await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points + ? WHERE id = ?'
  ).bind(points, userId).run();
}

export async function redeemPoints(userId: string, cost: number, env: Env) {
  const user = await env.DB.prepare(
    'SELECT loyalty_points FROM users WHERE id = ?'
  ).bind(userId).first();

  if (!user || user.loyalty_points < cost) {
    throw new Error('Insufficient points');
  }

  await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points - ? WHERE id = ? AND loyalty_points >= ?'
  ).bind(cost, userId, cost).run();
}
