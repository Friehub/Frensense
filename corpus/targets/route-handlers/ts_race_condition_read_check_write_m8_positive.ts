// [frensense]
// observation: Non-atomic read-check-write sequence to deduct credits or balance via an array element access.
// impact: Two concurrent requests can read the same balance and pass the check, enabling double-spend
// improvement: Use an atomic database update statement (UPDATE ... SET balance = balance - amount WHERE balance >= amount)
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

async function handlerA(req: Request, res: Response) {
    const arr = [env.KV];
    const stores = [env.KV]; const raw = await stores[0].get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await stores[0].put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [env.DB];
    const stores = [env.DB]; const raw = await stores[0].get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await stores[0].put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}
