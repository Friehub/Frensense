// SAFE: .then() chain with jwt.verify
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

function handlerA(req: Request, res: Response) {
    Promise.resolve(req.headers.authorization).then(val => {
        try { const payload = jwt.verify(val, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
    });
    res.json({ ok: true });
}

function handlerB(req: Request, res: Response) {
    new Promise(resolve => resolve(req.cookies.token)).then(val => {
        try { const payload = jwt.verify(val, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
    });
    res.json({ ok: true });
}
