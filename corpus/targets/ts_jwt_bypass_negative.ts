import jwt from "jsonwebtoken";

const SECRET = process.env.JWT_SECRET;

function authenticate(req: Request, res: Response) {
    const token = req.headers.authorization?.split(" ")[1];
    const payload = jwt.verify(token, SECRET);
    req.user = payload;
    next();
}

function verifySession(req: Request, res: Response) {
    const token = req.cookies.token;
    const decoded = jwt.verify(token, SECRET);
    if (!decoded) {
        return res.status(401).json({ error: "Invalid token" });
    }
    res.json({ userId: decoded.sub });
}

function getProfile(req: Request, res: Response) {
    const token = req.query.token;
    const data = jwt.verify(token as string, SECRET);
    res.json(data);
}
