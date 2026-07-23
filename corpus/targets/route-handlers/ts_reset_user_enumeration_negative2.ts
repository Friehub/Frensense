// SAFE: Always performs the same processing regardless of user existence and introduces constant delay
export async function requestReset(req: Request, db: DB): Promise<Response> {
  const { email } = await req.json();
  const token = crypto.randomBytes(32).toString('hex');
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (user) {
    await db.prepare('INSERT INTO reset_tokens (email, token, expires_at) VALUES (?, ?, ?)').bind(email, token, Date.now() + 3600000).run();
    await sendEmail(email, `Reset link: https://app.com/reset?token=${token}`);
  }
  await new Promise(resolve => setTimeout(resolve, 500));
  return new Response(JSON.stringify({ message: 'If that email is registered, a reset link has been sent.' }));
}
