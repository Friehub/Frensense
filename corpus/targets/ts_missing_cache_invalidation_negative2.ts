// SAFE alternative: write-through cache pattern
async function updateUserProfile(userId: string, data: any, db: DB, redis: Redis): Promise<void> {
  await db.query('UPDATE users SET name = $1, bio = $2 WHERE id = $3', [data.name, data.bio, userId]);
  // Write-through: update cache with new data
  const updated = await db.queryOne('SELECT * FROM users WHERE id = $1', [userId]);
  await redis.set(`user:${userId}`, JSON.stringify(updated), { EX: 3600 });
}
