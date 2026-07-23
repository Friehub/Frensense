// [frensense]
// observation: CAPTCHA validation is performed only on the client side, and the server accepts requests without verifying the CAPTCHA token.
// impact: An attacker can bypass the CAPTCHA by sending requests directly to the API endpoint without including the CAPTCHA token or by disabling JavaScript, enabling automated attacks.
// improvement: Always verify the CAPTCHA token server-side before processing the request. Reject requests with missing or invalid tokens.

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) {
    return new Response('Invalid credentials', { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
