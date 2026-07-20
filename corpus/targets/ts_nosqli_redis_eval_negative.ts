// SAFE: Removed EVAL with user-supplied scripts and used Redis built-in commands with parameterized keys and arguments.

import { createClient } from "redis";

const redis = createClient();

async function conditionalUpdate(req: Request, res: Response) {
    const key = req.params.key;
    const value = req.body.value;
    const current = await redis.get(key);
    if (current !== null) {
        await redis.set(key, JSON.stringify(value));
        res.json({ updated: true });
    } else {
        res.status(404).json({ error: "Key not found" });
    }
}

async function atomicIncrement(req: Request, res: Response) {
    const key = req.params.key;
    const result = await redis.incr(key);
    res.json({ count: result });
}
