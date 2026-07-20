// [frensense]
// observation: JWT token is decoded without verifying the signature across an async/await boundary.
// impact: An attacker can craft arbitrary JWTs.
// improvement: Always use jwt.verify() instead of jwt.decode()

import jwt from "jsonwebtoken";

async function getToken(req: any): Promise<string> { return req.headers.authorization; }
async function getCookie(req: any): Promise<string> { return req.cookies.token; }

async function handlerA(req: Request, res: Response) {
    const val = await getToken(req); const payload = jwt.decode(val); res.json(payload);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = await getCookie(req); const payload = jwt.decode(val); res.json(payload);
    res.json({ ok: true });
}
