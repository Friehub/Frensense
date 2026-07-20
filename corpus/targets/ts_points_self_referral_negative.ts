// SAFE: Verifies that the referrer and the referred user are different via multiple checks

export async function applyReferral(referralCode: string, newUserId: string, env: Env) {
  const referrer = await env.DB.prepare(
    'SELECT id, email FROM users WHERE referral_code = ?'
  ).bind(referralCode).first();

  if (!referrer) throw new Error('Invalid referral code');

  // SAFE: prevent self-referral
  if (referrer.id === newUserId) {
    throw new Error('Cannot refer yourself');
  }

  const newUser = await env.DB.prepare(
    'SELECT email, ip FROM users WHERE id = ?'
  ).bind(newUserId).first();

  if (newUser) {
    if (newUser.email === referrer.email) {
      throw new Error('Cannot use the same email for referral');
    }

    if (newUser.ip === referrer.ip) {
      throw new Error('Cannot refer from the same IP address');
    }
  }

  await env.DB.prepare(
    'INSERT INTO referrals (referrer_id, referred_id, bonus_awarded) VALUES (?, ?, ?)'
  ).bind(referrer.id, newUserId, 100).run();

  await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points + ? WHERE id = ?'
  ).bind(100, referrer.id).run();
}
