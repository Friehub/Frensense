// [frensense]
// observation: CSV export uses no character encoding or sanitisation — user input with commas, newlines, or quotes breaks the CSV structure and allows injection.
// impact: CSV injection via unescaped special characters. Malformed CSV rows cause data corruption, broken imports, and potential formula injection in spreadsheet software.
// improvement: Always quote fields containing special characters and escape embedded quotes.

import { Request, Response } from 'express';

export async function exportOrders(req: Request, res: Response): Promise<void> {
  const orders = [
    ['ORD-001', 'Laptop', '1200.00'],
    ['ORD-002', 'Monitor', '=SUM(A1:A10)'],
    ['ORD-003', 'Desk, "Executive"', '1500.00'],
  ];

  res.setHeader('Content-Type', 'text/csv');
  res.setHeader('Content-Disposition', 'attachment; filename="orders.csv"');

  let csv = 'id,product,amount\n';
  for (const row of orders) {
    csv += row.join(',') + '\n';
  }
  res.send(csv);
}
