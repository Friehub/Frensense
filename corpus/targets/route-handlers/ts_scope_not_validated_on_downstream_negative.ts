// SAFE: Downstream service verifies the token independently before trusting scopes
import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';

export async function deductInventory(req: Request, res: Response): Promise<void> {
  const token = req.headers.authorization?.split(' ')[1];
  if (!token) { res.status(401).json({ error: 'Missing token' }); return; }
  try {
    const decoded = jwt.verify(token, process.env.INVENTORY_JWT_SECRET!) as any;
    if (!decoded.scopes?.includes('inventory:deduct')) {
      res.status(403).json({ error: 'Insufficient scope for this service' });
      return;
    }
  } catch {
    res.status(401).json({ error: 'Invalid token' });
    return;
  }
  res.json({ deducted: true });
}
