// [frensense]
// observation: JWT token is decoded without verifying the signature, allowing forged tokens through a helper function.
// impact: An attacker can craft arbitrary JWTs with any payload and bypass authentication
// improvement: Always use jwt.verify() instead of jwt.decode() to validate the token signature
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.headers.authorization);
    const payload = jwt.decode(val); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.cookies.token);
    const payload = jwt.decode(val); res.json(payload);
    res.json({ ok: true });
}
