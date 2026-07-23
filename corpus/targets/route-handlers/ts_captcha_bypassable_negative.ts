// SAFE: CAPTCHA token is verified server-side before processing login
export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password, captchaToken } = await req.json();
  const captchaResult = await verifyCaptcha(captchaToken);
  if (!captchaResult.success) {
    return new Response('CAPTCHA verification failed', { status: 403 });
  }
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) {
    return new Response('Invalid credentials', { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}

async function verifyCaptcha(token: string): Promise<{ success: boolean }> {
  const res = await fetch('https://www.google.com/recaptcha/api/siteverify', {
    method: 'POST',
    body: `secret=${process.env.RECAPTCHA_SECRET}&response=${token}`,
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
  });
  return res.json();
}
