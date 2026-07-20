// SAFE: Verifies the aud claim matches the expected audience for this service
import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;
const API_AUDIENCE = 'https://api.frensense.io';

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const payload = jwt.verify(token, SECRET, { audience: API_AUDIENCE });
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
