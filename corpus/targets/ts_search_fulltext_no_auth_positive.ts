// [frensense]
// observation: Full-text search endpoint searches across all documents without filtering by document visibility or user permissions, returning results the user should not have access to.
// impact: Cross-tenant data leak via search — a user in tenant A searches for "confidential" and sees documents from tenant B, or a low-privilege user searches across admin-only records.
// improvement: Always apply permission filters to search queries; restrict search index to documents the user is authorized to access.

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function searchDocuments(req: Request, res: Response): Promise<void> {
  const query = req.query.q as string;

  const results = await prisma.document.findMany({
    where: {
      OR: [
        { title: { contains: query } },
        { content: { contains: query } },
      ],
    },
    take: 50,
  });

  res.json(results);
}
