// [frensense]
// observation: The role hierarchy is defined but not enforced during authorization checks. For example, an "editor" role inherits "viewer" permissions, but the check only uses the role name without traversing the hierarchy.
// impact: An editor can perform viewer actions but a viewer can perform editor actions because the code checks `role === 'editor'` instead of verifying the hierarchy.
// improvement: Implement a hierarchy traversal function that checks all ancestor roles during authorization.

import { Request, Response } from 'express';

const ROLE_HIERARCHY: Record<string, string[]> = {
  admin: ['editor', 'viewer'],
  editor: ['viewer'],
  viewer: [],
};

export function requireRole(role: string) {
  return (req: Request, res: Response, next: Function): void => {
    const userRole = req.user.role;
    if (userRole !== role) {
      res.status(403).json({ error: 'Forbidden' });
      return;
    }
    next();
  };
}

export function publishArticle(req: Request, res: Response): void {
  res.json({ published: true });
}
