// SAFE: Try-catch with jwt.verify
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
    try { const payload = jwt.verify(req.headers.authorization, SECRET); res.json(payload); } catch (err) { console.error(err); res.status(401).send("Invalid token"); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    try { const payload = jwt.verify(req.cookies.token, SECRET); res.json(payload); } catch (err) { console.error(err); res.status(401).send("Invalid token"); }
    res.json({ ok: true });
}
