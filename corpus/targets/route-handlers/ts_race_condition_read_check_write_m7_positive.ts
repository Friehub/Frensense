// [frensense]
// observation: Non-atomic read-check-write sequence to deduct credits or balance via destructured object property.
// impact: Two concurrent requests can read the same balance and pass the check, enabling double-spend
// improvement: Use an atomic database update statement (UPDATE ... SET balance = balance - amount WHERE balance >= amount)
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const { input } = env; const raw = await input.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await input.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const { input } = env; const raw = await input.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await input.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}
