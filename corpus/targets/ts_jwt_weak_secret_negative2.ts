// SAFE: Uses RS256 (asymmetric) so there is no shared secret that could be brute-forced
import jwt from 'jsonwebtoken';
import fs from 'fs';

const PRIVATE_KEY = fs.readFileSync('/etc/secrets/jwt-private.pem', 'utf-8');
const PUBLIC_KEY = fs.readFileSync('/etc/secrets/jwt-public.pem', 'utf-8');

export function issueToken(userId: string): string {
  return jwt.sign({ sub: userId }, PRIVATE_KEY, { algorithm: 'RS256', expiresIn: '1h' });
}

export function verifyToken(token: string): any {
  return jwt.verify(token, PUBLIC_KEY, { algorithms: ['RS256'] });
}
