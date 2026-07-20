// SAFE: Request body is validated with a zod schema before being used

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';
import { z } from 'zod';

const createUserSchema = z.object({
  email: z.string().email(),
  name: z.string().min(1).max(100),
  role: z.enum(['user', 'admin']).default('user'),
});

export async function POST(request: NextRequest) {
  const raw = await request.json();
  const parsed = createUserSchema.safeParse(raw);
  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.flatten() }, { status: 400 });
  }
  const user = await prisma.user.create({
    data: parsed.data,
  });
  return NextResponse.json(user);
}
