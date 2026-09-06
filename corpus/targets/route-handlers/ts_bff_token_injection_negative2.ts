// SAFE: BFF strips user-supplied Authorization headers and replaces them with a session-bound token
import { Request, Response, NextFunction } from 'express';

export function overrideAuthHeader(req: Request, res: Response, next: NextFunction): void {
  delete req.headers.authorization;
  const session = req.session as any;
  if (session?.userId) {
    const token = jwt.sign(
      { sub: session.userId },
      process.env.BFF_SESSION_SECRET!,
      { expiresIn: '5m' }
    );
    req.headers.authorization = `Bearer ${token}`;
  }
  next();
}

export async function bffProxy(req: Request, res: Response): Promise<void> {
  const upstream = await fetch('https://api.example.com/user/profile', {
    headers: { Authorization: req.headers.authorization! },
  });
  res.json(await upstream.json());
}
