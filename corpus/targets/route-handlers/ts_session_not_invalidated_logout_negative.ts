// SAFE: Server-side session is destroyed on logout
import session from 'express-session';

export async function logout(req: Request, res: Response, db: DB): Promise<void> {
  const token = req.cookies.token;
  await db.prepare('DELETE FROM sessions WHERE token = ?').bind(token).run();
  res.clearCookie('token');
  res.json({ success: true });
}

export async function apiLogout(req: Request, res: Response, db: DB): Promise<void> {
  const token = req.cookies.token;
  await db.prepare('UPDATE sessions SET revoked_at = ? WHERE token = ?').bind(Date.now(), token).run();
  res.setHeader('Set-Cookie', 'token=; Max-Age=0');
  res.json({ success: true });
}
