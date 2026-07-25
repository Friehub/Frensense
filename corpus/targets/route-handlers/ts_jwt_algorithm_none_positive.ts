// [frensense]
// observation: The JWT verification function uses an options object that allows the algorithm to be inferred from the token header, and does not reject tokens with alg: "none".
// impact: An attacker can forge a token with alg: "none" and arbitrary payload, bypassing authentication and gaining unauthorized access with any user identity.
// improvement: Explicitly specify the expected algorithm in jwt.verify() and reject algorithms that are not in the allowed list.
// cwe: CWE-345
// cvss: 9.1
// owasp: A02:2021
// severity: Critical

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
