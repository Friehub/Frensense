// SAFE: Manually checks the algorithm before verification
import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;
const ALLOWED_ALGS = ['HS256', 'HS384'];

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const decoded = jwt.decode(token, { complete: true });
    if (!decoded || typeof decoded === 'string' || !ALLOWED_ALGS.includes(decoded.header.alg)) {
      throw new Error('Unsupported algorithm');
    }
    const payload = jwt.verify(token, SECRET, { algorithms: ALLOWED_ALGS });
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
