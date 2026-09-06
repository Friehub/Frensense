// SAFE: Uses a middleware wrapper pattern that injects the authenticated user

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

type AuthenticatedHandler = (req: NextRequest, userId: string) => Promise<NextResponse>;

function requireAuth(handler: AuthenticatedHandler) {
  return async (request: NextRequest) => {
    const { auth } = await import('@/lib/auth');
    const session = await auth();
    if (!session?.user?.id) return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    return handler(request, session.user.id);
  };
}

export const GET = requireAuth(async (request: NextRequest, userId: string) => {
  const { searchParams } = new URL(request.url);
  const targetId = searchParams.get('userId') || userId;
  if (userId !== targetId && userId !== 'admin') {
    return NextResponse.json({ error: 'Forbidden' }, { status: 403 });
  }
  const orders = await prisma.order.findMany({ where: { userId: targetId } });
  return NextResponse.json(orders);
});
