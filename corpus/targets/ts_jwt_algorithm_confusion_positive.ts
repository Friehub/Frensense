// [frensense]
// observation: The application uses an RSA public key to verify tokens but does not restrict the algorithm, allowing an attacker to change alg from RS256 to HS256 and sign with the public key.
// impact: Since the public key is often accessible, an attacker can forge tokens by switching to symmetric HS256 and signing with the known public key as the secret.
// improvement: Always specify allowed algorithms in jwt.verify() and use separate secrets for symmetric and asymmetric modes.

import jwt from 'jsonwebtoken';

const PUBLIC_KEY = `-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----`;

export function authenticate(req: Request, res: Response): void {
  const token = req.headers.authorization?.split(' ')[1];
  try {
    const payload = jwt.verify(token, PUBLIC_KEY);
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
