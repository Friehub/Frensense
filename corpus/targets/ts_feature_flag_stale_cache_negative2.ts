// SAFE: No cache for feature flags — always reads from DB

import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function isFeatureEnabled(userId: string, feature: string): Promise<boolean> {
  const flag = await prisma.featureFlag.findUnique({ where: { name: feature } });
  return flag?.enabled ?? false;
}

export async function listEnabledFeatures(userId: string): Promise<string[]> {
  const flags = await prisma.featureFlag.findMany({ where: { enabled: true } });
  return flags.map((f) => f.name);
}
