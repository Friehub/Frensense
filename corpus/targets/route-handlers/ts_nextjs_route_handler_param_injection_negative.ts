// SAFE: Route param is used via parameterized query instead of string interpolation

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

interface Params {
  params: { slug: string };
}

export async function GET(request: NextRequest, { params }: Params) {
  const product = await prisma.product.findUnique({
    where: { slug: params.slug },
  });
  if (!product) return NextResponse.json({ error: 'Not found' }, { status: 404 });
  return NextResponse.json(product);
}
