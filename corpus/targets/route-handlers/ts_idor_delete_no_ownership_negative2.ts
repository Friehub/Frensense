// SAFE: Uses Prisma with ownership filter in delete
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function deleteDocument(req: Request): Promise<Response> {
  const session = getSession(req);
  const docId = req.params.id;
  try {
    await prisma.document.deleteMany({ where: { id: docId, ownerId: session.userId } });
    return new Response(JSON.stringify({ deleted: true }));
  } catch {
    return new Response('Not found', { status: 404 });
  }
}
