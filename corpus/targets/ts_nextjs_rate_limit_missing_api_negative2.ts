// SAFE: In-memory rate limiting with a simple sliding window per IP

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

const ipRequests = new Map<string, number[]>();

function checkRateLimit(ip: string, maxRequests: number, windowMs: number): boolean {
  const now = Date.now();
  const timestamps = ipRequests.get(ip) || [];
  const recent = timestamps.filter((t) => now - t < windowMs);
  if (recent.length >= maxRequests) return false;
  recent.push(now);
  ipRequests.set(ip, recent);
  return true;
}

export async function POST(request: NextRequest) {
  const ip = request.ip ?? request.headers.get('x-forwarded-for') ?? 'unknown';
  if (!checkRateLimit(ip, 5, 60_000)) {
    return NextResponse.json({ error: 'Too many requests' }, { status: 429 });
  }
  const { email, password } = await request.json();
  const user = await prisma.user.findUnique({ where: { email } });
  if (!user) return NextResponse.json({ error: 'Invalid credentials' }, { status: 401 });
  return NextResponse.json({ ok: true });
}
