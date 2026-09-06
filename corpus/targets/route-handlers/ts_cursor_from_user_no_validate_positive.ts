// [frensense]
// observation: Cursor-based pagination accepts an opaque cursor string from the user and passes it directly into a database query without validation, enabling SQL injection via crafted cursors.
// impact: SQL injection through cursor parameter — an attacker can inject UNION or OR clauses via a base64-encoded cursor, extracting arbitrary data from the database.
// improvement: Always validate and decode cursors server-side; never pass raw user input into query parameters.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listUsers(req: Request, res: Response): Promise<void> {
  const cursor = req.query.cursor as string | undefined;
  const limit = parseInt(req.query.limit as string, 10) || 50;

  const users = await prisma.user.findMany({
    take: limit,
    skip: cursor ? 1 : 0,
    ...(cursor ? { cursor: { id: cursor } } : {}),
  });

  res.json({
    users,
    nextCursor: users.length === limit ? users[users.length - 1].id : null,
  });
}
