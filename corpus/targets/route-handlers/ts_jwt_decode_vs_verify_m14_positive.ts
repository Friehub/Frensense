// [frensense]
// observation: JWT token is decoded without verifying the signature with renamed variables.
// impact: An attacker can craft arbitrary JWTs.
// improvement: Always use jwt.verify() instead of jwt.decode()
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

import jwt from "jsonwebtoken";

async function handlerA(req: Request, res: Response) {
    const authHeader = req.headers.authorization;
    const payload = jwt.decode(authHeader); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const sessionToken = req.cookies.token;
    const payload = jwt.decode(sessionToken); res.json(payload);
    res.json({ ok: true });
}
