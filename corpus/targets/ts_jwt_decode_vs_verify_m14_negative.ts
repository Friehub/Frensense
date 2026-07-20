// SAFE: Renamed variables with jwt.verify
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
    const authHeader = req.headers.authorization;
    try { const payload = jwt.verify(authHeader, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const sessionToken = req.cookies.token;
    try { const payload = jwt.verify(sessionToken, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
    res.json({ ok: true });
}
