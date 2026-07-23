// [frensense]
// observation: Non-atomic read-check-write sequence to deduct credits or balance via string concatenation.
// impact: Two concurrent requests can read the same balance and pass the check, enabling double-spend
// improvement: Use an atomic database update statement (UPDATE ... SET balance = balance - amount WHERE balance >= amount)

async function handlerA(req: Request, res: Response) {
    const raw = await env.KV.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await env.KV.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const raw = await env.DB.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await env.DB.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}
