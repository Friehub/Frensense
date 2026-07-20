// [frensense]
// observation: Users can earn referral points by referring themselves, either by creating a second account or by manipulating the referral link.
// impact: A user can create multiple accounts or use a self-referral trick to earn unlimited referral bonuses without bringing any new customers.
// improvement: Verify that the referrer and the referred user are different, using IP tracking, device fingerprinting, or email domain analysis.

export async function applyReferral(referralCode: string, newUserId: string, env: Env) {
  const referrer = await env.DB.prepare(
    'SELECT id FROM users WHERE referral_code = ?'
  ).bind(referralCode).first();

  if (!referrer) throw new Error('Invalid referral code');

  // VULNERABLE: no check that referrer and new user are different
  await env.DB.prepare(
    'INSERT INTO referrals (referrer_id, referred_id, bonus_awarded) VALUES (?, ?, ?)'
  ).bind(referrer.id, newUserId, 100).run();

  await env.DB.prepare(
    'UPDATE users SET loyalty_points = loyalty_points + ? WHERE id = ?'
  ).bind(100, referrer.id).run();
}
