// SAFE: Uses Prisma with ownership filter
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function shareDocument(req: Request): Promise<Response> {
  const session = getSession(req);
  const { docId, shareWithEmail } = await req.json();
  const doc = await prisma.document.findFirst({ where: { id: docId, ownerId: session.userId } });
  if (!doc) return new Response('Not found', { status: 404 });
  const targetUser = await prisma.user.findUnique({ where: { email: shareWithEmail } });
  if (!targetUser) return new Response('User not found', { status: 404 });
  await prisma.documentShare.create({ data: { documentId: docId, userId: targetUser.id } });
  return new Response(JSON.stringify({ shared: true }));
}
