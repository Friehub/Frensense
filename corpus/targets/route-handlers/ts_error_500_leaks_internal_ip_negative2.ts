// SAFE: JSON error response with request ID, no host/IP info.
import { Request, Response, NextFunction } from 'express';
import crypto from 'crypto';

export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction): void {
  const requestId = crypto.randomUUID();
  console.error(`[${requestId}] Unhandled error:`, err.message, err.stack);

  res.status(500).json({
    error: 'Internal server error',
    requestId,
  });
}
