// SAFE: BFF uses a session-derived service token, not a user-supplied token
import { Request, Response } from 'express';

export async function bffProxy(req: Request, res: Response): Promise<void> {
  const session = req.session as any;
  if (!session?.userId) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  const serviceToken = createServiceToken(session.userId, 'api.example.com');
  const upstream = await fetch('https://api.example.com/user/profile', {
    headers: { Authorization: `Bearer ${serviceToken}` },
  });
  res.json(await upstream.json());
}

function createServiceToken(userId: string, audience: string): string {
  return jwt.sign(
    { sub: userId, aud: audience, iss: 'bff' },
    process.env.BFF_SERVICE_SECRET!,
    { expiresIn: '5m' }
  );
}
