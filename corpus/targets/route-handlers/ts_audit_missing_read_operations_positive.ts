// [frensense]
// observation: The audit logging system only records write operations (create, update, delete) but ignores read operations. Sensitive data reads like medical records, financial statements, or PII are never logged.
// impact: Data exfiltration via read-only queries goes undetected. An attacker who gains read access can silently extract sensitive information without triggering any audit trail.
// improvement: Audit all read operations on sensitive resources (PII, financial, medical) in addition to writes.
// cwe: CWE-778
// cvss: 4.3
// owasp: A09:2021
// severity: Low

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function getMedicalRecord(req: Request, res: Response): Promise<void> {
  const record = await prisma.medicalRecord.findUnique({
    where: { id: req.params.id },
  });
  if (!record) { res.status(404).json({ error: 'Not found' }); return; }
  res.json(record);
}

export async function updateMedicalRecord(req: Request, res: Response): Promise<void> {
  await prisma.medicalRecord.update({
    where: { id: req.params.id },
    data: req.body,
  });
  await prisma.auditLog.create({
    data: {
      action: 'UPDATE',
      resource: 'medicalRecord',
      resourceId: req.params.id,
      userId: req.user.userId,
    },
  });
  res.json({ ok: true });
}
