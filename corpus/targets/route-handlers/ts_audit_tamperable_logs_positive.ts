// [frensense]
// observation: Audit logs are stored in the same database table that the application writes to during normal operations, allowing an attacker with write access to modify or delete audit records.
// impact: An attacker who compromises a database account can cover their tracks by deleting or altering log entries, eliminating forensic evidence of their actions.
// improvement: Write audit logs to an append-only store (immutable log, separate database, or a service with different credentials) that the application cannot modify or delete.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function auditAction(userId: string, action: string, resource: string): Promise<void> {
  await prisma.auditLog.create({
    data: { userId, action, resource, timestamp: new Date() },
  });
}

export async function deleteAuditLogs(req: Request, res: Response): Promise<void> {
  const { olderThan } = req.body;
  await prisma.auditLog.deleteMany({
    where: { timestamp: { lt: new Date(olderThan) } },
  });
  res.json({ deleted: true });
}
