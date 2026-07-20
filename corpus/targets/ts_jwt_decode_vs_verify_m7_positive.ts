// [frensense]
// observation: JWT token is decoded without verifying the signature, allowing forged tokens via destructured object property.
// impact: An attacker can craft arbitrary JWTs with any payload and bypass authentication
// improvement: Always use jwt.verify() instead of jwt.decode() to validate the token signature

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const { input } = req.headers; const payload = jwt.decode(input?.replace("Bearer ", "")); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const { value } = req.cookies; const payload = jwt.decode(value); res.json(payload);
    res.json({ ok: true });
}
