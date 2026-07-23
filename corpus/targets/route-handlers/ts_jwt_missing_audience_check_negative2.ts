// SAFE: Manual audience check after verification for multi-audience support
import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;
const ALLOWED_AUDIENCES = ['https://api.frensense.io', 'https://admin.frensense.io'];

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const payload = jwt.verify(token, SECRET);
    if (!payload.aud || !ALLOWED_AUDIENCES.includes(payload.aud as string)) {
      throw new Error('Invalid audience');
    }
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
