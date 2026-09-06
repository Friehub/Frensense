// SAFE: Uses a read-through proxy that automatically audits reads on sensitive collections
import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function auditedFindUnique(model: string, args: any, userId: string): Promise<any> {
  const result = await (prisma as any)[model].findUnique(args);
  if (result) {
    await prisma.auditLog.create({
      data: {
        action: 'READ',
        resource: model,
        resourceId: args.where.id,
        userId,
      },
    });
  }
  return result;
}

export async function getMedicalRecord(req: Request, res: Response): Promise<void> {
  const record = await auditedFindUnique('medicalRecord', {
    where: { id: req.params.id },
  }, req.user.userId);
  if (!record) { res.status(404).json({ error: 'Not found' }); return; }
  res.json(record);
}
