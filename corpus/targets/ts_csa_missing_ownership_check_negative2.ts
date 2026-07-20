// SAFE: Ownership check performed with Prisma, ensuring the user owns the resource before reading
import { PrismaClient } from '@prisma/client';

const prisma = new PrismaClient();

export async function handleReadFile(req: any, env: any) {
  const session = getSession(req);
  const projectId = req.query.projectId;
  const path = req.query.path;
  const project = await prisma.project.findFirst({
    where: { id: projectId, ownerId: session.userId }
  });
  if (!project) return new Response('Forbidden', { status: 403 });
  const file = await prisma.file.findFirst({
    where: { projectId, path, project: { ownerId: session.userId } }
  });
  if (!file) return new Response('Not found', { status: 404 });
  return new Response(file.content);
}
