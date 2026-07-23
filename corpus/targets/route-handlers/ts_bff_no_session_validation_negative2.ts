// SAFE: Session requires re-authentication for high-risk actions using step-up auth
import { Request, Response, NextFunction } from 'express';

export async function requireStepUpIfSuspicious(req: Request, res: Response, next: NextFunction): Promise<void> {
  const session = req.session as any;
  if (!session?.userId) { res.status(401).json({ error: 'Unauthorized' }); return; }
  const ipChanged = session.lastIp && session.lastIp !== req.ip;
  const uaChanged = session.lastUa && session.lastUa !== req.headers['user-agent'];
  if (ipChanged || uaChanged) {
    session.requireStepUp = true;
  }
  session.lastIp = req.ip;
  session.lastUa = req.headers['user-agent'];
  next();
}

export async function bffHandler(req: Request, res: Response): Promise<void> {
  const session = req.session as any;
  if (session.requireStepUp && !session.stepUpVerified) {
    res.status(403).json({ error: 'Step-up authentication required' });
    return;
  }
  res.json(await fetchUpstreamData(session.userId));
}
