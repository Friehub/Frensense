// SAFE: Always performs bcrypt comparison regardless of user existence
import bcrypt from 'bcrypt';

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  const hash = user?.passwordHash || '$2b$10$placeholderxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx';
  if (!await bcrypt.compare(password, hash)) {
    return new Response('Invalid credentials', { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
