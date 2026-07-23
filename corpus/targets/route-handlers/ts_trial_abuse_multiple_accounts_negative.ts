// SAFE: Tracks trial eligibility by email, IP, device fingerprint, and payment method

export async function startTrial(req: Request, env: Env) {
  const { email, deviceFingerprint } = await req.json() as { email: string; deviceFingerprint: string };
  const ip = req.headers.get('CF-Connecting-IP') || req.headers.get('x-forwarded-for');

  // SAFE: check multiple dimensions
  const existingUser = await env.DB.prepare(
    'SELECT id FROM users WHERE (email = ? OR ip = ? OR device_fingerprint = ?) AND trial_used = 1'
  ).bind(email, ip, deviceFingerprint).first();

  if (existingUser) {
    throw new Error('Trial already used for this account or device');
  }

  // SAFE: require a payment method to start trial
  const { paymentMethodId } = await req.json() as { paymentMethodId: string };
  if (!paymentMethodId) {
    throw new Error('Payment method required for trial');
  }

  await env.DB.prepare(
    'INSERT INTO users (email, ip, device_fingerprint, payment_method_id, trial_used, created_at) VALUES (?, ?, ?, ?, ?, ?)'
  ).bind(email, ip, deviceFingerprint, paymentMethodId, 1, Date.now()).run();

  return { trialStarted: true };
}
