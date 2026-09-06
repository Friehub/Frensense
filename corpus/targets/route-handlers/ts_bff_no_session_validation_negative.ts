// SAFE: Session is bound to IP and user-agent, validated on every BFF request
import { Request, Response, NextFunction } from 'express';

export function validateSessionBinding(req: Request, res: Response, next: NextFunction): void {
  const session = req.session as any;
  if (!session?.userId) {
    res.status(401).json({ error: 'No session' });
    return;
  }
  if (session.boundIp !== req.ip || session.boundUa !== req.headers['user-agent']) {
    req.session.destroy(() => {});
    res.status(401).json({ error: 'Session binding mismatch' });
    return;
  }
  next();
}

export async function bffHandler(req: Request, res: Response): Promise<void> {
  const data = await fetchUpstreamData(req.session.userId);
  res.json(data);
}
