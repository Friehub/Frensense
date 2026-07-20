// [frensense]
// observation: JWT token is decoded without verifying the signature via a promise .then() chain.
// impact: An attacker can craft arbitrary JWTs.
// improvement: Always use jwt.verify() instead of jwt.decode()

import jwt from "jsonwebtoken";

function handlerA(req: Request, res: Response) {
    Promise.resolve(req.headers.authorization).then(val => {
        const payload = jwt.decode(val); res.json(payload);
    });
    res.json({ ok: true });
}

function handlerB(req: Request, res: Response) {
    new Promise(resolve => resolve(req.cookies.token)).then(val => {
        const payload = jwt.decode(val); res.json(payload);
    });
    res.json({ ok: true });
}
