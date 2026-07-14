// SAFE variant 7: distributed lock pattern with expiry
async function deductCreditsWithLock(userId: string, amount: number, kv: KVNamespace) {
  const lockKey = `lock:credits:${userId}`;
  const acquired = await kv.put(lockKey, '1', { expirationTtl: 10, condition: 'not-exist' });
  if (!acquired) throw new Error('CONCURRENT_MODIFICATION');
  try {
    const raw = await kv.get(`credits:${userId}`);
    const balance = raw ? parseInt(raw, 10) : 0;
    if (balance < amount) return false;
    await kv.put(`credits:${userId}`, String(balance - amount));
    return true;
  } finally {
    await kv.delete(lockKey);
  }
}
