// SAFE: Async path with jwt.verify and Bearer prefix
import jwt from "jsonwebtoken";
const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function getToken(req: any): Promise<string> { return req.headers.authorization.replace("Bearer ", ""); }
async function handlerA(req: Request, res: Response) {
  try { const val = await getToken(req); const payload = jwt.verify(val, SECRET); res.json(payload); } catch { res.status(401).send("Invalid token"); }
}
