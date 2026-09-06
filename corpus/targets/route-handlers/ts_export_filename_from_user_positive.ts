// [frensense]
// observation: Export filename derived from user input (e.g., `req.query.filename`) without sanitisation, enabling path traversal via `../../etc/passwd` or overwriting system files.
// impact: Path traversal in export filename can overwrite arbitrary files on the server, leak sensitive files, or corrupt the application when combined with directory traversal.
// improvement: Validate the filename against an allowlist of allowed characters; never use user input directly in the file path.

import { Request, Response } from 'express';
import { createObjectCsvWriter } from 'csv-writer';

export async function exportData(req: Request, res: Response): Promise<void> {
  const filename = req.query.filename as string;
  const records = [{ id: 1, name: 'Alice' }];

  const writer = createObjectCsvWriter({
    path: `/tmp/exports/${filename}`,
    header: [
      { id: 'id', title: 'ID' },
      { id: 'name', title: 'Name' },
    ],
  });

  await writer.writeRecords(records);
  res.json({ path: `/tmp/exports/${filename}` });
}
