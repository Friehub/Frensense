// [frensense]
// observation: JWT token is decoded without verifying the signature, allowing forged tokens via string concatenation.
// impact: An attacker can craft arbitrary JWTs with any payload and bypass authentication
// improvement: Always use jwt.verify() instead of jwt.decode() to validate the token signature

async function handlerA(req: Request, res: Response) {
    const token = req.headers.authorization?.replace("Bearer ", ""); const payload = jwt.decode(token); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const token = req.cookies.token; const payload = jwt.decode(token); res.json(payload);
    res.json({ ok: true });
}
