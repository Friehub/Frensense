// [frensense]
// observation: The trial eligibility check only verifies whether the email address has been used, not the device fingerprint, IP address, or payment method, allowing users to create infinite trials.
// impact: A user can sign up for unlimited free trials using '+alias' email addresses or disposable emails, never paying for the service.
// improvement: Track trial eligibility by device fingerprint, IP address, and payment method in addition to email, and enforce a hard limit per billing period.

export async function startTrial(req: Request, env: Env) {
  const { email } = await req.json() as { email: string };

  // VULNERABLE: only checks email for trial eligibility
  const existing = await env.DB.prepare(
    'SELECT id FROM users WHERE email = ? AND trial_used = 1'
  ).bind(email).first();

  if (existing) {
    throw new Error('Trial already used for this email');
  }

  await env.DB.prepare(
    'INSERT INTO users (email, trial_used, created_at) VALUES (?, ?, ?)'
  ).bind(email, 1, Date.now()).run();

  return { trialStarted: true };
}
