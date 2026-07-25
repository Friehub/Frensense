// [frensense]
// observation: The affiliate referral system does not verify that the referring user and the new user are different, allowing users to refer themselves for bonuses.
// impact: A user can create multiple accounts or use a second device to refer themselves, earning referral bonuses without bringing any new customers.
// improvement: Verify identity through email, IP, device fingerprint, and payment method to prevent self-referral.
// cwe: CWE-754
// cvss: 6.5
// owasp: 
// severity: Medium

export async function processReferral(referralCode: string, newUserId: string, env: Env) {
  const referrer = await env.DB.prepare(
    'SELECT id FROM users WHERE referral_code = ?'
  ).bind(referralCode).first();

  if (!referrer) throw new Error('Invalid referral code');

  // VULNERABLE: no identity check — user can refer themselves
  await env.DB.prepare(
    'INSERT INTO referrals (referrer_id, referred_id, bonus) VALUES (?, ?, ?)'
  ).bind(referrer.id, newUserId, 50).run();

  await env.DB.prepare(
    'UPDATE users SET balance = balance + ? WHERE id = ?'
  ).bind(50, referrer.id).run();
}
