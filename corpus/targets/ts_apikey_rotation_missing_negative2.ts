// SAFE: Rotation enforced via key versioning — old versions are auto-revoked
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();
const MAX_KEY_VERSION = 3;

export async function rotateApiKey(userId: string): Promise<string> {
  const key = `sk-${crypto.randomUUID().replace(/-/g, '')}`;
  const version = await prisma.apiKey.count({ where: { userId } }) + 1;
  await prisma.apiKey.create({
    data: {
      userId,
      keyHash: hashKey(key),
      version,
    },
  });
  const stale = await prisma.apiKey.findMany({
    where: { userId, version: { lte: version - MAX_KEY_VERSION } },
  });
  if (stale.length) {
    await prisma.apiKey.updateMany({
      where: { id: { in: stale.map(s => s.id) } },
      data: { revoked: true },
    });
  }
  return key;
}
