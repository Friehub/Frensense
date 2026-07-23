// [frensense]
// observation: The password reset endpoint returns different responses for registered vs unregistered email addresses.
// impact: An attacker can enumerate valid user accounts by observing the response message or timing difference, which can then be targeted for credential stuffing or phishing.
// improvement: Return identical responses for both existing and non-existing accounts. Consider using a generic message like "If the account exists, a reset link has been sent."

export async function requestReset(req: Request, db: DB): Promise<Response> {
  const { email } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) {
    return new Response(JSON.stringify({ error: 'Email not found' }), { status: 404 });
  }
  const token = crypto.randomBytes(32).toString('hex');
  await db.prepare('INSERT INTO reset_tokens (email, token, expires_at) VALUES (?, ?, ?)').bind(email, token, Date.now() + 3600000).run();
  await sendEmail(email, `Reset link: https://app.com/reset?token=${token}`);
  return new Response(JSON.stringify({ message: 'Reset link sent' }));
}
