// SAFE: Returns identical response regardless of whether the email exists
export async function requestReset(req: Request, db: DB): Promise<Response> {
  const { email } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (user) {
    const token = crypto.randomBytes(32).toString('hex');
    await db.prepare('INSERT INTO reset_tokens (email, token, expires_at) VALUES (?, ?, ?)').bind(email, token, Date.now() + 3600000).run();
    await sendEmail(email, `Reset link: https://app.com/reset?token=${token}`);
  }
  return new Response(JSON.stringify({ message: 'If that email is registered, a reset link has been sent.' }));
}
