// SAFE: Route param is validated for allowed characters and used with Prisma's parameterized raw query

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

interface Params {
  params: { slug: string };
}

function isValidSlug(slug: string): boolean {
  return /^[a-zA-Z0-9\-_]+$/.test(slug) && slug.length <= 200;
}

export async function GET(request: NextRequest, { params }: Params) {
  if (!isValidSlug(params.slug)) {
    return NextResponse.json({ error: 'Invalid slug' }, { status: 400 });
  }
  const product = await prisma.$queryRawUnsafe(
    'SELECT * FROM products WHERE slug = $1',
    params.slug
  );
  return NextResponse.json(product);
}
