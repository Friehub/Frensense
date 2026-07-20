// SAFE: Restricts verification to RS256 only, preventing algorithm confusion
import jwt from 'jsonwebtoken';

const PUBLIC_KEY = `-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----`;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const payload = jwt.verify(token, PUBLIC_KEY, { algorithms: ['RS256'] });
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
