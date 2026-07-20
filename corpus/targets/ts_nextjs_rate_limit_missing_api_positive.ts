// [frensense]
// observation: A sensitive API route (login, checkout, password reset) has no rate limiting, allowing unlimited requests.
// impact: Attackers can brute-force credentials, submit unlimited orders, or perform denial-of-wallet attacks.
// improvement: Apply rate limiting to all sensitive API endpoints.

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

export async function POST(request: NextRequest) {
  const { email, password } = await request.json();
  const user = await prisma.user.findUnique({ where: { email } });
  if (!user) return NextResponse.json({ error: 'Invalid credentials' }, { status: 401 });
  return NextResponse.json({ ok: true });
}
