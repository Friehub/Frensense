// SAFE: Explicitly restricts allowed algorithms — "none" is rejected
import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const payload = jwt.verify(token, SECRET, { algorithms: ['HS256'] });
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
