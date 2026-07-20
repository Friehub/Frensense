// SAFE: Topological sort at startup rejects hierarchical cycles before any role checks run
import { Request, Response } from 'express';

const ROLE_HIERARCHY: Record<string, string[]> = {
  admin: ['manager'],
  manager: ['editor'],
  editor: ['viewer'],
  viewer: [],
};

function validateHierarchy(hierarchy: Record<string, string[]>): void {
  const visited = new Set<string>();
  const stack = new Set<string>();
  function dfs(node: string): void {
    if (stack.has(node)) throw new Error('Cycle detected in role hierarchy');
    if (visited.has(node)) return;
    visited.add(node);
    stack.add(node);
    for (const child of hierarchy[node] ?? []) {
      dfs(child);
    }
    stack.delete(node);
  }
  for (const role of Object.keys(hierarchy)) {
    dfs(role);
  }
}

validateHierarchy(ROLE_HIERARCHY);

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
