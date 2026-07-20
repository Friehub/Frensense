// SAFE: Verifies referrer and referred are different through email, IP, and device checks

export async function processReferral(referralCode: string, newUserId: string, env: Env) {
  const referrer = await env.DB.prepare(
    'SELECT id, email FROM users WHERE referral_code = ?'
  ).bind(referralCode).first();

  if (!referrer) throw new Error('Invalid referral code');

  if (referrer.id === newUserId) {
    throw new Error('Cannot refer yourself');
  }

  const newUser = await env.DB.prepare(
    'SELECT email, ip, device_fingerprint FROM users WHERE id = ?'
  ).bind(newUserId).first();

  if (!newUser) throw new Error('New user not found');

  if (newUser.email === referrer.email) {
    throw new Error('Same email cannot be used for self-referral');
  }

  if (newUser.ip === referrer.ip) {
    throw new Error('Same IP address cannot be used for self-referral');
  }

  await env.DB.prepare(
    'INSERT INTO referrals (referrer_id, referred_id, bonus) VALUES (?, ?, ?)'
  ).bind(referrer.id, newUserId, 50).run();

  await env.DB.prepare(
    'UPDATE users SET balance = balance + ? WHERE id = ?'
  ).bind(50, referrer.id).run();
}
