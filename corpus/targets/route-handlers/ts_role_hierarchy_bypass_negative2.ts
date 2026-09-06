// SAFE: Uses a flat permission set resolved from the hierarchy at login, avoiding hierarchy traversal at check time
import { Request, Response, NextFunction } from 'express';

const ROLE_PERMISSIONS: Record<string, string[]> = {
  admin: ['read', 'write', 'publish', 'delete'],
  editor: ['read', 'write', 'publish'],
  viewer: ['read'],
};

function resolvePermissions(userRole: string): Set<string> {
  return new Set(ROLE_PERMISSIONS[userRole] ?? []);
}

export function requirePermission(perm: string) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const permissions = resolvePermissions(req.user.role);
    if (!permissions.has(perm)) {
      res.status(403).json({ error: 'Missing permission' });
      return;
    }
    next();
  };
}

export function publishArticle(req: Request, res: Response): void {
  res.json({ published: true });
}
