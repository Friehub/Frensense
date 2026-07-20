// SAFE: Manual type checking and field whitelisting instead of spreading the body directly

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

function validateUserInput(body: unknown): { email: string; name: string; role: string } {
  if (typeof body !== 'object' || body === null) throw new Error('Invalid body');
  const obj = body as Record<string, unknown>;
  if (typeof obj.email !== 'string' || !obj.email.includes('@')) throw new Error('Invalid email');
  if (typeof obj.name !== 'string' || obj.name.length === 0) throw new Error('Invalid name');
  const role = obj.role === 'admin' ? 'admin' : 'user';
  return { email: obj.email, name: obj.name, role };
}

export async function POST(request: NextRequest) {
  const raw = await request.json();
  const data = validateUserInput(raw);
  const user = await prisma.user.create({ data });
  return NextResponse.json(user);
}
