// SAFE: .then() chain with jwt.verify and Bearer prefix
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

function handlerA(req: Request, res: Response) {
  Promise.resolve(req.headers.authorization).then(val => {
    try { const token = val.replace("Bearer ", ""); const payload = jwt.verify(token, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
  });
  res.json({ ok: true });
}
