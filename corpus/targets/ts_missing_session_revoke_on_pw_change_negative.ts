// SAFE: invalidate sessions on credential change
async function changePassword(userId: string, newPassword: string, db: DB, sessionStore: SessionStore): Promise<void> {
  const hash = await bcrypt.hash(newPassword, 12);
  await db.transaction(async (tx) => {
    await tx.query('UPDATE users SET password_hash = $1 WHERE id = $2', [hash, userId]);
    // SAFE: increment session version to invalidate all sessions
    await tx.query('UPDATE users SET session_version = session_version + 1 WHERE id = $1', [userId]);
  });
  await sessionStore.destroyAll(userId);
}

async function changeEmail(userId: string, newEmail: string, db: DB, redis: Redis): Promise<void> {
  await db.query('UPDATE users SET email = $1 WHERE id = $2', [newEmail, userId]);
  // SAFE: revoke session tokens
  const sessions = await redis.keys(`session:${userId}:*`);
  if (sessions.length > 0) await redis.del(...sessions);
}
