// SAFE: CAPTCHA token verified with hCaptcha (alternative provider)
export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password, hcaptchaToken } = await req.json();
  const formData = new FormData();
  formData.append('secret', process.env.HCAPTCHA_SECRET!);
  formData.append('response', hcaptchaToken);
  const res = await fetch('https://hcaptcha.com/siteverify', { method: 'POST', body: formData });
  const result = await res.json();
  if (!result.success) return new Response('CAPTCHA verification failed', { status: 403 });
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) return new Response('Invalid credentials', { status: 401 });
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
