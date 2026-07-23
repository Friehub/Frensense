// [frensense]
// observation: JWT token is decoded without verifying the signature, allowing forged tokens through an object property.
// impact: An attacker can craft arbitrary JWTs with any payload and bypass authentication.
// improvement: Always use jwt.verify() instead of jwt.decode() to validate the token signature

import jwt from "jsonwebtoken";

async function handlerA(req: Request, res: Response) {
    const auth = { header: req.headers.authorization };
    const payload = jwt.decode(auth.header); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const auth = { cookie: req.cookies.token };
    const payload = jwt.decode(auth.cookie); res.json(payload);
    res.json({ ok: true });
}
