// SAFE: Uses separate key pair per algorithm mode, explicit algorithm check
import jwt from 'jsonwebtoken';

const PUBLIC_KEY = `-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----`;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const header = jwt.decode(token, { complete: true })?.header;
    if (!header || header.alg !== 'RS256') {
      throw new Error('Only RS256 is supported');
    }
    const payload = jwt.verify(token, PUBLIC_KEY, { algorithms: ['RS256'] });
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
