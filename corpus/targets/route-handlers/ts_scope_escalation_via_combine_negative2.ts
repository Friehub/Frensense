// SAFE: Scope combinations are validated against an allowlist; unknown combinations are rejected
import { Request, Response, NextFunction } from 'express';

const ALLOWED_COMBOS: Record<string, string[]> = {
  'read:orders+write:returns': ['read:orders', 'write:returns'],
};

export function checkAccess(req: Request, res: Response, next: NextFunction): void {
  const scopeSets: string[][] = req.user.scopeSets;
  const flat = scopeSets.flat().sort().join('+');
  const allowed = ALLOWED_COMBOS[flat];
  if (!allowed) {
    res.status(403).json({ error: 'Scope combination not allowed' });
    return;
  }
  req.scopes = allowed;
  next();
}
