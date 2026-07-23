// SAFE: Uses a token version stored in the database, checked on every request
import { Request, Response, NextFunction } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function requireValidToken(req: Request, res: Response, next: NextFunction): Promise<void> {
  const authHeader = req.headers.authorization;
  if (!authHeader) { res.status(401).json({ error: 'Missing token' }); return; }
  const token = authHeader.split(' ')[1];
  const decoded = jwt.verify(token, process.env.JWT_SECRET!) as { sub: string; version: number };
  const user = await prisma.user.findUnique({ where: { id: decoded.sub } });
  if (!user || user.tokenVersion !== decoded.version) {
    res.status(401).json({ error: 'Token invalidated' });
    return;
  }
  req.user = decoded;
  next();
}
