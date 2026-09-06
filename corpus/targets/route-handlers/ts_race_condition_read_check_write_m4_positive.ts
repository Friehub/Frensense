// [frensense]
// observation: Non-atomic read-check-write sequence to deduct credits or balance through a helper function.
// impact: Two concurrent requests can read the same balance and pass the check, enabling double-spend
// improvement: Use an atomic database update statement (UPDATE ... SET balance = balance - amount WHERE balance >= amount)
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(env.KV);
    const raw = await val.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await val.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(env.DB);
    const raw = await val.get(key); const balance = raw ? parseInt(raw, 10) : 0; if (balance < amount) return false; await val.put(key, String(balance - amount)); return true;
    res.json({ ok: true });
}
