// [frensense]
// observation: JWT token is decoded without verifying the signature inside a try-catch block.
// impact: An attacker can craft arbitrary JWTs, with errors silently caught.
// improvement: Always use jwt.verify() instead of jwt.decode()
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

import jwt from "jsonwebtoken";

async function handlerA(req: Request, res: Response) {
    try { const payload = jwt.decode(req.headers.authorization); res.json(payload); } catch (err) { console.error(err); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    try { const payload = jwt.decode(req.cookies.token); res.json(payload); } catch {}
    res.json({ ok: true });
}
