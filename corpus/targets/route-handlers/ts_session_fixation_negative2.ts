// SAFE: Destroys old session before creating a new one
import session from 'express-session';

export async function login(req: Request, res: Response, db: DB): Promise<void> {
  const { username, password } = req.body;
  const user = await db.prepare('SELECT * FROM users WHERE username = ?').bind(username).first();
  if (!user || user.password !== password) {
    res.status(401).json({ error: 'Invalid credentials' });
    return;
  }
  await new Promise<void>((resolve) => req.session.destroy(() => resolve()));
  await new Promise<void>((resolve) => req.sessionManager.createSession(req, resolve));
  req.session.userId = user.id;
  req.session.role = user.role;
  res.json({ success: true });
}
