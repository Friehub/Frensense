// SAFE: Conditional branch with jwt.verify
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
    if (req.headers.authorization) {
        try { const payload = jwt.verify(req.headers.authorization, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
    } else { res.status(401).send("No token"); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    if (req.cookies.token) {
        try { const payload = jwt.verify(req.cookies.token, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
    }
    res.json({ ok: true });
}
