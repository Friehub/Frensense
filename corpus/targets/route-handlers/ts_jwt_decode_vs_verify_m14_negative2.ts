// SAFE: Renamed variables with jwt.verify and Bearer prefix
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
  const authHeader = req.headers.authorization;
  try { const token = authHeader.replace("Bearer ", ""); const payload = jwt.verify(token, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
  res.json({ ok: true });
}
