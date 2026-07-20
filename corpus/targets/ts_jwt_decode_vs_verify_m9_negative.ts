// SAFE: Object property verified with jwt.verify
import jwt from "jsonwebtoken";

const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
    const auth = { header: req.headers.authorization };
    try {
        const payload = jwt.verify(auth.header, SECRET); res.json(payload);
    } catch { res.status(401).send("Invalid token"); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const auth = { cookie: req.cookies.token };
    try {
        const payload = jwt.verify(auth.cookie, SECRET); res.json(payload);
    } catch { res.status(401).send("Invalid token"); }
    res.json({ ok: true });
}
