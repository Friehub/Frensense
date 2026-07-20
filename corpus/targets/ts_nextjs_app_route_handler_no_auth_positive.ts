// [frensense]
// observation: An App Router route handler function (GET/POST/PUT/DELETE) performs database operations without any authentication.
// impact: The endpoint is publicly accessible and any client can read or write sensitive data.
// improvement: Add authentication checks at the top of every App Router route handler.

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const userId = searchParams.get('userId');
  const orders = await prisma.order.findMany({ where: { userId } });
  return NextResponse.json(orders);
}
