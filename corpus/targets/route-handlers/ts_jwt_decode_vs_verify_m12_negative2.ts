// SAFE: Try-catch with jwt.verify and Bearer prefix
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
  try { const token = req.headers.authorization.replace("Bearer ", ""); const payload = jwt.verify(token, SECRET); res.json(payload); } catch (err) { res.status(401).json({ error: "Invalid token" }); }
  res.json({ ok: true });
}
