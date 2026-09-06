// [frensense]
// observation: Pagination endpoint returns the total count of all matching records (`total: 15000`) alongside paginated results. If the query filters by user-scoped data but the total leaks cross-tenant counts, an attacker can infer hidden data volume.
// impact: Information disclosure — the total count reveals the existence and volume of data the user should not know about (e.g., "how many flagged accounts exist", "total revenue of competitor tenants").
// improvement: Return total only for non-sensitive listings, or cap the total at a maximum display value.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Request, Response } from 'express';
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function listInvoices(req: Request, res: Response): Promise<void> {
  const page = parseInt(req.query.page as string, 10) || 1;
  const limit = 50;
  const skip = (page - 1) * limit;

  const [invoices, total] = await Promise.all([
    prisma.invoice.findMany({ skip, take: limit, where: { userId: req.session.userId } }),
    prisma.invoice.count({ where: { userId: req.session.userId } }),
  ]);

  res.json({ invoices, total, page });
}
