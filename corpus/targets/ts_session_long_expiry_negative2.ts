// SAFE: Session with configurable short expiry, stored server-side
export function createSession(db: DB, userId: string): Promise<string> {
  const sessionId = crypto.randomUUID();
  const expiresAt = Date.now() + 60 * 60 * 1000;
  return db.prepare('INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?)').bind(sessionId, userId, expiresAt).run().then(() => sessionId);
}

export async function getSession(db: DB, sessionId: string) {
  const session = await db.prepare('SELECT * FROM sessions WHERE id = ? AND expires_at > ?').bind(sessionId, Date.now()).first();
  return session || null;
}
