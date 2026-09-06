// SAFE: The role hierarchy is traversed to check ancestor roles during authorization
import { Request, Response, NextFunction } from 'express';

const ROLE_HIERARCHY: Record<string, string[]> = {
  admin: ['editor', 'viewer'],
  editor: ['viewer'],
  viewer: [],
};

function hasRole(userRole: string, requiredRole: string): boolean {
  if (userRole === requiredRole) return true;
  const inherited = ROLE_HIERARCHY[requiredRole] ?? [];
  return inherited.some(r => hasRole(userRole, r));
}

export function requireRole(role: string) {
  return (req: Request, res: Response, next: NextFunction): void => {
    if (!hasRole(req.user.role, role)) {
      res.status(403).json({ error: 'Forbidden' });
      return;
    }
    next();
  };
}

export function publishArticle(req: Request, res: Response): void {
  res.json({ published: true });
}
