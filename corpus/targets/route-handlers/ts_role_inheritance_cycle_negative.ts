// SAFE: Cycle detection with a visited set prevents infinite recursion
import { Request, Response } from 'express';

const ROLE_HIERARCHY: Record<string, string[]> = {
  admin: ['manager'],
  manager: ['admin'],
  user: [],
};

export function hasRole(userRole: string, requiredRole: string): boolean {
  return hasRoleWithGuard(userRole, requiredRole, new Set<string>());
}

function hasRoleWithGuard(userRole: string, requiredRole: string, visited: Set<string>): boolean {
  if (userRole === requiredRole) return true;
  if (visited.has(requiredRole)) return false;
  visited.add(requiredRole);
  const inherited = ROLE_HIERARCHY[requiredRole] ?? [];
  return inherited.some(r => hasRoleWithGuard(userRole, r, visited));
}

export function checkAccess(req: Request, res: Response): void {
  if (hasRole(req.user.role, 'admin')) {
    res.json({ access: 'granted' });
  } else {
    res.status(403).json({ error: 'Forbidden' });
  }
}
