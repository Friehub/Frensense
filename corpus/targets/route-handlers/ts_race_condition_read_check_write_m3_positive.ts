// [frensense]
// observation: Non-atomic read-check-write sequence to deduct credits or balance through multiple variable assignments.
// impact: Two concurrent requests can read the same balance and pass the check, enabling double-spend
// improvement: Use an atomic database update statement (UPDATE ... SET balance = balance - amount WHERE balance >= amount)

async function handlerA(req: Request, res: Response) {
    const a = env.KV;
    const b = a;
    const raw = await b.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await b.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = env.DB;
    const y = x;
    const z = y;
    const raw = await z.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await z.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}
