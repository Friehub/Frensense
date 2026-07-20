// SAFE: Manually handle child record deletion with confirmation

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deleteOrganization(orgId: string, confirmCascade: boolean) {
  if (!confirmCascade) {
    throw new Error('Cascade deletion not confirmed');
  }

  await prisma.organization.update({
    where: { id: orgId },
    data: { deletedAt: new Date() },
  });
}
