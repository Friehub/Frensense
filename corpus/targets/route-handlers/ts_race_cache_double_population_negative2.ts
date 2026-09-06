// SAFE: Uses a dedicated cache-aside pattern with a distributed mutex (Redis SET NX) to prevent double computation
import express from "express";

async function withCacheLock<T>(key: string, compute: () => Promise<T>, ttl: number = 300): Promise<T> {
    const cached = await cache.get(key);
    if (cached) return JSON.parse(cached);
    const lockKey = `__lock__${key}`;
    const acquired = await cache.set(lockKey, "1", "NX", "EX", 10);
    if (acquired === "OK") {
        try {
            const value = await compute();
            await cache.set(key, JSON.stringify(value), "EX", ttl);
            return value;
        } finally {
            await cache.del(lockKey);
        }
    } else {
        await new Promise(r => setTimeout(r, 50));
        return withCacheLock(key, compute, ttl);
    }
}

export async function getUserData(req: express.Request, res: express.Response) {
    const data = await withCacheLock(`user:${req.params.userId}`, async () => {
        return db.query("SELECT * FROM users WHERE id = ?", [req.params.userId]);
    });
    res.json(data);
}

export async function getExpensiveReport(req: express.Request, res: express.Response) {
    const data = await withCacheLock(`report:${req.params.reportId}`, async () => {
        return computeExpensiveReport(req.params.reportId);
    });
    res.json(data);
}
