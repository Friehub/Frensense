// [frensense]
// observation: The JWT is decoded with jwt.decode() which does not validate any claims including exp, then manually used for access control.
// impact: An expired token remains valid indefinitely. An attacker can use a stolen token even after it should have expired.
// improvement: Use jwt.verify() which automatically validates exp, nbf, and iss claims. If manual validation is needed, check exp >= current timestamp.

import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  const payload = jwt.decode(token);
  if (!payload || typeof payload === 'string') {
    res.status(401).json({ error: 'Invalid token' });
    return;
  }
  req.user = payload;
  next();
}
