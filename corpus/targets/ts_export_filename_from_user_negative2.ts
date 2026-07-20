// SAFE: Server generates the filename, user input only indicates export type.
import { Request, Response } from 'express';
import { createObjectCsvWriter } from 'csv-writer';
import path from 'path';
import crypto from 'crypto';

export async function exportData(req: Request, res: Response): Promise<void> {
  const exportType = req.query.type as string;

  const allowedTypes = ['users', 'orders', 'products'] as const;
  if (!allowedTypes.includes(exportType as any)) {
    res.status(400).json({ error: 'invalid export type' });
    return;
  }

  const suffix = crypto.randomBytes(4).toString('hex');
  const filename = `${exportType}_${suffix}.csv`;
  const filePath = path.join('/tmp/exports', filename);

  const records = [{ id: 1, name: 'Alice' }];
  const writer = createObjectCsvWriter({
    path: filePath,
    header: [
      { id: 'id', title: 'ID' },
      { id: 'name', title: 'Name' },
    ],
  });

  await writer.writeRecords(records);
  res.json({ path: filePath });
}
