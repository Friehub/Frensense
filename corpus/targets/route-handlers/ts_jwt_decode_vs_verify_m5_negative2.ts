// SAFE: Uses jwt.verify with audience and issuer checks
import jwt from "jsonwebtoken";
function verifyToken(token: string): any {
    return jwt.verify(token, process.env.JWT_SECRET, { audience: process.env.JWT_AUDIENCE, issuer: process.env.JWT_ISSUER });
}
function handlerA(req: Request, res: Response) {
    const token = req.headers.authorization?.replace("Bearer ", "");
    if (!token) return res.status(401).json({ error: "No token" });
    try { const payload = verifyToken(token); res.json(payload); }
    catch { res.status(401).json({ error: "Invalid token" }); }
}
function handlerB(req: Request, res: Response) {
    const token = req.cookies.token;
    if (!token) return res.status(401).json({ error: "No token" });
    try { const payload = verifyToken(token); res.json(payload); }
    catch { res.status(401).json({ error: "Invalid token" }); }
}
