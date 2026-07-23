// SAFE: Generic error response in production; details logged server-side only.
import { Request, Response, NextFunction } from 'express';

export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction): void {
  console.error('Unhandled error:', {
    message: err.message,
    stack: err.stack,
    url: req.originalUrl,
    method: req.method,
  });

  res.status(500).json({
    error: 'An unexpected error occurred. Please try again later.',
    requestId: req.headers['x-request-id'] || 'unknown',
  });
}

export async function getUser(req: Request, res: Response): Promise<void> {
  try {
    const user = await prisma.user.findUnique({ where: { id: req.params.id } });
    if (!user) {
      res.status(404).json({ error: 'User not found' });
      return;
    }
    res.json(user);
  } catch (err) {
    console.error('Database error in getUser:', err);
    res.status(500).json({ error: 'An unexpected error occurred' });
  }
}

const prisma = { user: { findUnique: async (_: any) => null } };
