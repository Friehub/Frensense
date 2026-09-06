// SAFE: JWT blacklist approach — token added to denylist on logout
import { Redis } from 'ioredis';

const redis = new Redis();

export async function logout(req: Request, res: Response): Promise<void> {
  const token = req.headers.authorization?.split(' ')[1];
  if (token) {
    const exp = extractExp(token);
    const ttl = exp ? exp - Math.floor(Date.now() / 1000) : 86400;
    await redis.set(`blacklist:${token}`, 'true', 'EX', ttl);
  }
  res.json({ success: true });
}

function extractExp(token: string): number | null {
  try {
    const payload = JSON.parse(Buffer.from(token.split('.')[1], 'base64').toString());
    return payload.exp || null;
  } catch { return null; }
}
