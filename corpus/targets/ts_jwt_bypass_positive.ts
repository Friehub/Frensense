import jwt from "jsonwebtoken";

function authenticate(req: Request, res: Response) {
    const token = req.headers.authorization?.split(" ")[1];
    const payload = jwt.decode(token);
    req.user = payload;
    next();
}

function verifySession(req: Request, res: Response) {
    const token = req.cookies.token;
    const decoded = jwt.decode(token);
    if (!decoded) {
        return res.status(401).json({ error: "Invalid token" });
    }
    res.json({ userId: decoded.sub });
}

function getProfile(req: Request, res: Response) {
    const token = req.query.token;
    const data = jwt.decode(token as string);
    res.json(data);
}
