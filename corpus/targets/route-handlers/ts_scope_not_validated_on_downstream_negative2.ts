// SAFE: Uses scoped service tokens (not user token forwarding) for service-to-service auth
import { Request, Response } from 'express';

export async function deductInventory(req: Request, res: Response): Promise<void> {
  const serviceToken = req.headers['x-service-token'];
  if (serviceToken !== process.env.INVENTORY_SERVICE_SECRET) {
    res.status(403).json({ error: 'Invalid service token' });
    return;
  }
  const { orderId, items } = req.body;
  res.json({ deducted: true, orderId });
}
