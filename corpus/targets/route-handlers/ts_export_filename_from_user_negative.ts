// SAFE: Filename sanitised against an allowlist before use.
import { Request, Response } from 'express';
import { createObjectCsvWriter } from 'csv-writer';
import path from 'path';

const ALLOWED_FILENAME = /^[a-zA-Z0-9_\-]+\.csv$/;

export async function exportData(req: Request, res: Response): Promise<void> {
  const filename = req.query.filename as string;

  if (!ALLOWED_FILENAME.test(filename)) {
    res.status(400).json({ error: 'invalid filename' });
    return;
  }

  const records = [{ id: 1, name: 'Alice' }];
  const filePath = path.join('/tmp/exports', filename);

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
