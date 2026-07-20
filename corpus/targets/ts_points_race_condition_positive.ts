// [frensense]
// observation: Points balance is read, checked, and written as separate operations, creating a race condition where two concurrent redemptions can both succeed.
// impact: A user can redeem points for two items simultaneously, spending the same points twice and causing double payout.
// improvement: Use an atomic decrement operation for points deduction, similar to inventory stock deduction.

export async function redeemPoints(userId: string, cost: number, env: Env) {
  // VULNERABLE: read-check-write race condition
  const user = await env.DB.prepare(
    'SELECT loyalty_points FROM users WHERE id = ?'
  ).bind(userId).first();

  if (!user || user.loyalty_points < cost) {
    throw new Error('Insufficient points');
  }

  const newBalance = user.loyalty_points - cost;
  await env.DB.prepare(
    'UPDATE users SET loyalty_points = ? WHERE id = ?'
  ).bind(newBalance, userId).run();

  await env.DB.prepare(
    'INSERT INTO point_redemptions (user_id, cost) VALUES (?, ?)'
  ).bind(userId, cost).run();
}
