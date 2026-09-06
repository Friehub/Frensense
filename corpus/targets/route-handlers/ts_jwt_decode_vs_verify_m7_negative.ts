// SAFE: Implements safe alternative
// SAFE: Uses jwt.verify() with a secret to validate the token signature
import jwt from "jsonwebtoken";
function handlerA(req: Request, res: Response) {
    const token = req.headers.authorization?.replace("Bearer ", "");
    if (!token) return res.status(401).json({ error: "No token" });
    try { const payload = jwt.verify(token, process.env.JWT_SECRET); res.json(payload); }
    catch { res.status(401).json({ error: "Invalid token" }); }
}
function handlerB(req: Request, res: Response) {
    const token = req.cookies.token;
    if (!token) return res.status(401).json({ error: "No token" });
    try { const payload = jwt.verify(token, process.env.JWT_SECRET); res.json(payload); }
    catch { res.status(401).json({ error: "Invalid token" }); }
}
