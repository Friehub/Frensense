// [frensense]
// observation: JWT token is decoded without verifying the signature, allowing forged tokens via a template literal interpolation.
// impact: An attacker can craft arbitrary JWTs with any payload and bypass authentication
// improvement: Always use jwt.verify() instead of jwt.decode() to validate the token signature
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

async function handlerA(req: Request, res: Response) {
    const payload = jwt.decode(req.headers.authorization?.replace("Bearer ", "")); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const payload = jwt.decode(req.cookies.token); res.json(payload);
    res.json({ ok: true });
}
