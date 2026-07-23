// SAFE: Session ID is regenerated after successful login to prevent fixation
import session from 'express-session';

export async function login(req: Request, res: Response, db: DB): Promise<void> {
  const { username, password } = await req.body;
  const user = await db.prepare('SELECT * FROM users WHERE username = ?').bind(username).first();
  if (!user || user.password !== password) {
    res.status(401).json({ error: 'Invalid credentials' });
    return;
  }
  await new Promise<void>((resolve) => req.session.regenerate(() => resolve()));
  req.session.userId = user.id;
  req.session.role = user.role;
  res.json({ success: true });
}
