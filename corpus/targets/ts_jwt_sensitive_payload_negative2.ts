// SAFE: Uses opaque session identifier in the JWT; all user data stored server-side
import jwt from 'jsonwebtoken';
import crypto from 'crypto';

export async function createSession(user: User, db: DB): Promise<string> {
  const sessionId = crypto.randomUUID();
  await db.prepare('INSERT INTO sessions (id, user_id, email, role, created_at) VALUES (?, ?, ?, ?, ?)')
    .bind(sessionId, user.id, user.email, user.role, Date.now()).run();
  return jwt.sign({ sid: sessionId }, process.env.JWT_SECRET!, { expiresIn: '1h' });
}

export async function getSessionFromToken(token: string, db: DB): Promise<any> {
  const { sid } = jwt.verify(token, process.env.JWT_SECRET!) as any;
  return db.prepare('SELECT * FROM sessions WHERE id = ?').bind(sid).first();
}
