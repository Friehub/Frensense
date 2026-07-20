// SAFE: Audit logs are written to a separate append-only store via a dedicated service
import { Request, Response } from 'express';

const AUDIT_SERVICE_URL = process.env.AUDIT_SERVICE_URL!;

export async function auditAction(userId: string, action: string, resource: string): Promise<void> {
  await fetch(`${AUDIT_SERVICE_URL}/logs`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'X-Audit-Secret': process.env.AUDIT_SECRET! },
    body: JSON.stringify({ userId, action, resource, timestamp: new Date().toISOString() }),
  });
}

export async function deleteAuditLogs(req: Request, res: Response): Promise<void> {
  res.status(403).json({ error: 'Audit logs are immutable' });
}
