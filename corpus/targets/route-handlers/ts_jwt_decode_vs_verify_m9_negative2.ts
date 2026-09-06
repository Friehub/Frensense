// SAFE: Object property verified with jwt.verify and Bearer prefix handling
import jwt from "jsonwebtoken";

const SECRET = process.env.JWT_SECRET || "fallback-secret";

async function handlerA(req: Request, res: Response) {
  const auth = { header: req.headers.authorization };
  try {
    const payload = jwt.verify(auth.header.replace("Bearer ", ""), SECRET);
    res.json(payload);
  } catch { res.status(401).send("Invalid token"); }
}
