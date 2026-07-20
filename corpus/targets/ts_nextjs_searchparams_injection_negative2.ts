// SAFE: SearchParams are coerced to primitive types and validated with a schema

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';
import { z } from 'zod';

const searchSchema = z.object({
  q: z.string().min(1).max(100),
  page: z.coerce.number().int().positive().default(1),
});

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const raw = Object.fromEntries(searchParams.entries());
  const parsed = searchSchema.safeParse(raw);
  if (!parsed.success) {
    return NextResponse.json({ error: 'Invalid query parameters' }, { status: 400 });
  }
  const users = await prisma.user.findMany({
    where: {
      email: { contains: parsed.data.q },
    },
    skip: (parsed.data.page - 1) * 20,
    take: 20,
  });
  return NextResponse.json(users);
}
