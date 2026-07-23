// SAFE: JWT payload contains only the user identifier; sensitive data fetched server-side
import jwt from 'jsonwebtoken';

export function issueToken(user: User): string {
  return jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
}

export async function getCurrentUser(req: Request, db: DB): Promise<User | null> {
  const userId = req.user.sub;
  return db.prepare('SELECT * FROM users WHERE id = ?').bind(userId).first();
}
