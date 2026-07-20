// SAFE: JWT secret is a cryptographically random, long string from environment config
import jwt from 'jsonwebtoken';

const SECRET = process.env.JWT_SECRET!;
if (!SECRET || SECRET.length < 32) {
  throw new Error('JWT_SECRET must be at least 32 characters');
}

export function issueToken(userId: string): string {
  return jwt.sign({ sub: userId }, SECRET, { expiresIn: '1h' });
}

export function verifyToken(token: string): any {
  return jwt.verify(token, SECRET, { algorithms: ['HS256'] });
}
