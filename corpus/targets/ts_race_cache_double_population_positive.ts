// [frensense]
// observation: A cache check (key lookup) and cache write (key set) are not atomic. Two concurrent requests for the same uncached data both miss the cache, both compute the value, and both write to the cache, causing duplicate work and potential data inconsistency.
// impact: Cache stampede — multiple concurrent requests repeat expensive computation (DB query, API call) after a cache miss, wasting resources and potentially returning stale data.
// improvement: Use an atomic SETNX (set-if-not-exists) operation to ensure only one request computes and writes the cache value.

import express from "express";

export async function getUserData(req: express.Request, res: express.Response) {
    const userId = req.params.userId;
    const cacheKey = `user:${userId}`;
    const cached = await cache.get(cacheKey);
    if (cached) {
        return res.json(JSON.parse(cached));
    }
    const userData = await db.query("SELECT * FROM users WHERE id = ?", [userId]);
    await cache.set(cacheKey, JSON.stringify(userData));
    res.json(userData);
}

export async function getExpensiveReport(req: express.Request, res: express.Response) {
    const reportId = req.params.reportId;
    const cached = await cache.get(`report:${reportId}`);
    if (cached) return res.json(JSON.parse(cached));
    const data = await computeExpensiveReport(reportId);
    await cache.set(`report:${reportId}`, JSON.stringify(data));
    res.json(data);
}
