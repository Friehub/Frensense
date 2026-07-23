// [frensense]
// observation: JWT token is decoded without verifying the signature inside a conditional block on the tainted branch.
// impact: An attacker can craft arbitrary JWTs.
// improvement: Always use jwt.verify() instead of jwt.decode()

import jwt from "jsonwebtoken";

async function handlerA(req: Request, res: Response) {
    if (req.headers.authorization) {
        const payload = jwt.decode(req.headers.authorization); res.json(payload);
    } else { res.status(401).send("No token"); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    if (req.cookies.token) {
        const payload = jwt.decode(req.cookies.token); res.json(payload);
    }
    res.json({ ok: true });
}
