// SAFE: Prisma with tenant-scoped client
import { PrismaClient } from '@prisma/client';

function createTenantScopedClient(tenantId: string): PrismaClient {
  const client = new PrismaClient();
  client.$use(async (params, next) => {
    if (params.model && params.action === 'findMany' || params.action === 'findFirst') {
      params.args.where = { ...params.args.where, tenantId };
    }
    return next(params);
  });
  return client;
}

export async function getUsers(req: Request): Promise<Response> {
  const session = getSession(req);
  const prisma = createTenantScopedClient(session.tenantId);
  const users = await prisma.user.findMany({ select: { id: true, name: true, email: true } });
  return new Response(JSON.stringify(users));
}
