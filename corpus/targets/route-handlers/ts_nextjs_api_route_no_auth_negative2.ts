// SAFE: Uses an API route wrapper that enforces authentication

import type { NextApiRequest, NextApiResponse } from 'next';
import prisma from '@/lib/prisma';

function withAuth(handler: (req: NextApiRequest, res: NextApiResponse, userId: string) => Promise<void>) {
  return async (req: NextApiRequest, res: NextApiResponse) => {
    const { getServerSession } = await import('next-auth');
    const { authOptions } = await import('@/lib/auth');
    const session = await getServerSession(req, res, authOptions);
    if (!session?.user?.id) return res.status(401).json({ error: 'Unauthorized' });
    return handler(req, res, session.user.id);
  };
}

export default withAuth(async (req, res, userId) => {
  const { userId: targetId } = req.query;
  if (userId !== targetId && userId !== 'admin') {
    return res.status(403).json({ error: 'Forbidden' });
  }
  const user = await prisma.user.findUnique({ where: { id: targetId as string } });
  res.json(user);
});
