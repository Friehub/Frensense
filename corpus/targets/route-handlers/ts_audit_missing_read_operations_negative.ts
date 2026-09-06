// SAFE: Read operations on sensitive resources are also audited
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

const SENSITIVE_MODELS = ['medicalRecord', 'financialDocument', 'personalInfo'];

export async function getMedicalRecord(req: Request, res: Response): Promise<void> {
  const record = await prisma.medicalRecord.findUnique({
    where: { id: req.params.id },
  });
  if (!record) { res.status(404).json({ error: 'Not found' }); return; }
  await prisma.auditLog.create({
    data: {
      action: 'READ',
      resource: 'medicalRecord',
      resourceId: req.params.id,
      userId: req.user.userId,
    },
  });
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
