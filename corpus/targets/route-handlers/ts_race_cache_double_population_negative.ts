// SAFE: Uses SETNX (set if not exists) with a TTL to ensure only the first request computes and populates the cache
import express from "express";

export async function getUserData(req: express.Request, res: express.Response) {
    const userId = req.params.userId;
    const cacheKey = `user:${userId}`;
    const cached = await cache.get(cacheKey);
    if (cached) {
        return res.json(JSON.parse(cached));
    }
    const lockKey = `lock:${cacheKey}`;
    const acquired = await cache.setnx(lockKey, "1", 10);
    if (acquired) {
        try {
            const userData = await db.query("SELECT * FROM users WHERE id = ?", [userId]);
            await cache.set(cacheKey, JSON.stringify(userData), 300);
            res.json(userData);
        } finally {
            await cache.del(lockKey);
        }
    } else {
        await new Promise(r => setTimeout(r, 100));
        const retry = await cache.get(cacheKey);
        if (retry) return res.json(JSON.parse(retry));
        return getUserData(req, res);
    }
}
