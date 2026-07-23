// SAFE: Manual validation with explicit exp check against current time
import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const decoded = jwt.decode(token) as any;
    if (!decoded || decoded.exp < Math.floor(Date.now() / 1000)) {
      throw new Error('Token expired');
    }
    const payload = jwt.verify(token, SECRET, { algorithms: ['HS256'] });
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
