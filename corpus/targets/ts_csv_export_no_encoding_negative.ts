// SAFE: Fields with special characters are quoted and embedded quotes are escaped.
import { Request, Response } from 'express';

function encodeCsvField(value: string): string {
  if (value.includes(',') || value.includes('"') || value.includes('\n') || value.includes('\r')) {
    return '"' + value.replace(/"/g, '""') + '"';
  }
  return value;
}

export async function exportOrders(req: Request, res: Response): Promise<void> {
  const orders = [
    ['ORD-001', 'Laptop', '1200.00'],
    ['ORD-002', 'Monitor', '=SUM(A1:A10)'],
    ['ORD-003', 'Desk, "Executive"', '1500.00'],
  ];

  res.setHeader('Content-Type', 'text/csv; charset=utf-8');
  res.setHeader('Content-Disposition', 'attachment; filename="orders.csv"');

  const header = 'id,product,amount\n';
  const rows = orders.map(row => row.map(encodeCsvField).join(',')).join('\n');
  res.send(header + rows);
}
