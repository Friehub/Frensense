// SAFE: Uses BigInt for point calculations and checks for overflow

export async function awardPoints(userId: string, points: number, env: Env) {
  const pointsBig = BigInt(points);

  const user = await env.DB.prepare(
    'SELECT loyalty_points FROM users WHERE id = ?'
  ).bind(userId).first();

  const currentBig = BigInt(user?.loyalty_points ?? 0);
  const newBalance = currentBig + pointsBig;

  if (newBalance > BigInt('9223372036854775807')) {
    throw new Error('Points balance would overflow');
  }

  await env.DB.prepare(
    'UPDATE users SET loyalty_points = ? WHERE id = ?'
  ).bind(Number(newBalance), userId).run();
}

export async function redeemPoints(userId: string, cost: number, env: Env) {
  const result = await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points - ? WHERE id = ? AND loyalty_points >= ?'
  ).bind(cost, userId, cost).run();

  if (result.meta.changes === 0) {
    throw new Error('Insufficient points');
  }
}
