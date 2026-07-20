// [frensense]
// observation: JWT is verified with jwt.verify() but the aud (audience) claim is not validated, so a token issued for one service can be used against another.
// impact: A token intended for Service A can be replayed against Service B, enabling cross-service impersonation in a microservices architecture.
// improvement: Pass the expected audience value in the jwt.verify() options to ensure the token was issued for the correct recipient.

import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const payload = jwt.verify(token, SECRET);
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
