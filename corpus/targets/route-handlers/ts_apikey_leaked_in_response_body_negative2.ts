// SAFE: Error sanitization middleware strips secrets from all response bodies
import { Request, Response, NextFunction } from 'express';

const SENSITIVE_PATTERNS = [/sk-[a-z0-9]+/gi, /Bearer\s+[A-Za-z0-9\-._~+/]+/g];

export function sanitizeErrorBody(err: Error, req: Request, res: Response, next: NextFunction): void {
  let body = err.message;
  for (const pattern of SENSITIVE_PATTERNS) {
    body = body.replace(pattern, '[REDACTED]');
  }
  res.status(500).json({ error: body });
}
