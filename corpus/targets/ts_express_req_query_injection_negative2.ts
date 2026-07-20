// SAFE: Validate input type and use an ORM with safe query construction.

import express from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const app = express();

app.get('/api/users', async (req, res) => {
  const rawId = req.query.id;
  const id = typeof rawId === 'string' ? Number(rawId) : Number.NaN;
  if (Number.isNaN(id)) {
    res.status(400).json({ error: 'Invalid id' });
    return;
  }
  const users = await prisma.user.findMany({ where: { id } });
  res.json(users);
});
