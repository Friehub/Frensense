// [frensense]
// observation: Database record updated but the corresponding cache entry is not invalidated or updated.
// impact: Stale data served from cache after updates. Users see outdated information until the cache TTL expires. For critical data (pricing, inventory, user status), this causes consistency bugs and incorrect business decisions.
// improvement: Invalidate related cache keys whenever a record is updated, or use cache-aside (write-through) pattern.

async function updateUserProfile(userId: string, data: any, db: DB, redis: Redis): Promise<void> {
  // VULNERABLE: DB updated but cache not invalidated
  await db.query('UPDATE users SET name = $1, bio = $2 WHERE id = $3', [data.name, data.bio, userId]);
}

async function updateProductPrice(productId: string, price: number, db: DB): Promise<void> {
  // VULNERABLE: cached price becomes stale
  await db.query('UPDATE products SET price = $1 WHERE id = $2', [price, productId]);
}
