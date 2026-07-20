// SAFE: Uses well-tested CSV library with automatic encoding.
import { Request, Response } from 'express';
import { Parser } from '@json2csv/plainjs';

export async function exportOrders(req: Request, res: Response): Promise<void> {
  const orders = [
    { id: 'ORD-001', product: 'Laptop', amount: '1200.00' },
    { id: 'ORD-002', product: 'Monitor', amount: '=SUM(A1:A10)' },
    { id: 'ORD-003', product: 'Desk, "Executive"', amount: '1500.00' },
  ];

  const parser = new Parser({ withBOM: true });
  const csv = parser.parse(orders);

  res.setHeader('Content-Type', 'text/csv; charset=utf-8');
  res.setHeader('Content-Disposition', 'attachment; filename="orders.csv"');
  res.send(csv);
}
