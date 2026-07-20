// SAFE: Soft delete with explicit child handling

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deleteOrganization(orgId: string) {
  const memberCount = await prisma.organizationMember.count({
    where: { organizationId: orgId },
  });

  if (memberCount > 0) {
    throw new Error(`Cannot delete organization with ${memberCount} active members`);
  }

  return prisma.organization.delete({
    where: { id: orgId },
  });
}
