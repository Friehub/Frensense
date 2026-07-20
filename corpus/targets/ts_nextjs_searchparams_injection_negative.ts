// SAFE: SearchParams value is validated and sanitized before use in database query

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

function sanitizeSearchTerm(term: string | null): string {
  if (!term) return '';
  return term.replace(/[^a-zA-Z0-9\s\-_.@]/g, '').slice(0, 100);
}

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const q = sanitizeSearchTerm(searchParams.get('q'));
  if (q.length < 1) {
    return NextResponse.json({ error: 'Invalid search term' }, { status: 400 });
  }
  const users = await prisma.user.findMany({
    where: {
      email: { contains: q },
    },
  });
  return NextResponse.json(users);
}
