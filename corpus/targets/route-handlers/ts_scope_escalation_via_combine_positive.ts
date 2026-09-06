// [frensense]
// observation: The authorization system grants scopes additively — combining two low-privilege scopes produces a permission set that includes high-privilege capabilities neither scope individually possessed.
// impact: An attacker with access to two restricted API keys can combine their scopes to perform operations that should require admin-level authorization.
// improvement: Validate scopes independently rather than merging them. Implement an allowlist that explicitly defines which scope combinations are valid.

import { Request, Response } from 'express';

interface AuthScope {
  resource: string;
  action: string;
}

export function combineScopes(scopeSets: AuthScope[][]): AuthScope[] {
  const merged = new Map<string, AuthScope>();
  for (const set of scopeSets) {
    for (const scope of set) {
      merged.set(`${scope.resource}:${scope.action}`, scope);
    }
  }
  return Array.from(merged.values());
}

export function checkAccess(req: Request, res: Response, next: Function): void {
  const tokenScopes: AuthScope[][] = req.user.scopeSets;
  const effectiveScopes = combineScopes(tokenScopes);
  req.effectiveScopes = effectiveScopes;
  next();
}
