// SAFE: invalidate cache after DB update
async function updateUserProfile(userId: string, data: any, db: DB, redis: Redis): Promise<void> {
  await db.query('UPDATE users SET name = $1, bio = $2 WHERE id = $3', [data.name, data.bio, userId]);
  await redis.del(`user:${userId}`);
  await redis.del(`user:${userId}:profile`);
}

async function updateProductPrice(productId: string, price: number, db: DB, redis: Redis): Promise<void> {
  await db.query('UPDATE products SET price = $1 WHERE id = $2', [price, productId]);
  await redis.del(`product:${productId}`);
}
