// [frensense]
// observation: Production Express error handler returns the full error object including stack trace, query parameters, and file paths in the response body.
// impact: Information disclosure — an attacker can trigger errors to learn database table names, internal IPs, file system structure, and framework versions, enabling targeted exploits.
// improvement: Log detailed errors server-side; return only a generic message to the client in production.

import { Request, Response, NextFunction } from 'express';

export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction): void {
  console.log('Error details:', err);
  res.status(500).json({
    error: err.message,
    stack: err.stack,
    query: req.query,
    body: req.body,
    path: req.path,
  });
}

export async function getUser(req: Request, res: Response): Promise<void> {
  const user = await prisma.user.findUnique({ where: { id: req.params.id } });
  if (!user) {
    throw new Error(`User not found with id ${req.params.id}`);
  }
  res.json(user);
}

const prisma = { user: { findUnique: async (_: any) => null } };
