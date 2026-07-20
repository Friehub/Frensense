// SAFE: Uses scrypt with appropriate parameters as an alternative to bcrypt
import crypto from 'crypto';
import { promisify } from 'util';

const scryptAsync = promisify(crypto.scrypt);

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const salt = crypto.randomBytes(16).toString('hex');
  const derivedKey = await scryptAsync(password, salt, 64) as Buffer;
  const hash = `${salt}:${derivedKey.toString('hex')}`;
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
