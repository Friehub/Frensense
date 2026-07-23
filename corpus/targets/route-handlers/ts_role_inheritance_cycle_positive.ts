// [frensense]
// observation: The role inheritance graph contains a cycle (e.g., admin inherits from manager, manager inherits from admin), causing the role resolution function to recurse infinitely.
// impact: Any authorization check that traverses the hierarchy results in a stack overflow or infinite loop, crashing the server or hanging the request.
// improvement: Implement cycle detection when building the role hierarchy or use a topological sort. Limit recursion depth.

import { Request, Response } from 'express';

const ROLE_HIERARCHY: Record<string, string[]> = {
  admin: ['manager'],
  manager: ['admin'],
  user: [],
};

export function hasRole(userRole: string, requiredRole: string): boolean {
  if (userRole === requiredRole) return true;
  const inherited = ROLE_HIERARCHY[requiredRole] ?? [];
  return inherited.some(r => hasRole(userRole, r));
}

export function checkAccess(req: Request, res: Response): void {
  if (hasRole(req.user.role, 'admin')) {
    res.json({ access: 'granted' });
  } else {
    res.status(403).json({ error: 'Forbidden' });
  }
}
