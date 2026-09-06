// SAFE: Uses NODE_ENV to conditionally show detailed errors in dev only.
import { Request, Response, NextFunction } from 'express';

export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction): void {
  const isDev = process.env.NODE_ENV === 'development';

  if (isDev) {
    console.error('Error:', err);
    res.status(500).json({
      error: err.message,
      stack: err.stack,
    });
  } else {
    console.error('Unhandled error:', {
      message: err.message,
      requestId: req.id,
    });
    res.status(500).json({
      error: 'Internal server error',
      requestId: req.id,
    });
  }
}
