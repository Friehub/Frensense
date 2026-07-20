// [frensense]
// observation: Prisma schema uses onDelete: Cascade without explicit confirmation that cascading deletes are intended and safe.
// impact: Deleting a parent record silently deletes all related child records, potentially causing irreversible data loss.
// improvement: Use onDelete: Restrict or SetNull by default, or add a comment/check confirming cascade is intentional.

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deleteOrganization(orgId: string) {
  return prisma.organization.delete({
    where: { id: orgId },
  });
}
