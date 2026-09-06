// SAFE: Scopes are intersected, not merged — the user only gets permissions common to all scope sets
import { Request, Response, NextFunction } from 'express';

export function checkAccess(req: Request, res: Response, next: NextFunction): void {
  const scopeSets: string[][] = req.user.scopeSets;
  if (!scopeSets || scopeSets.length === 0) {
    res.status(403).json({ error: 'No scopes' });
    return;
  }
  const common = scopeSets.reduce((acc, set) =>
    acc.filter(s => set.includes(s))
  );
  req.scopes = common;
  next();
}
