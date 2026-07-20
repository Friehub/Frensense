// [frensense]
// observation: App Router route handler uses a dynamic route param directly in a database query without sanitization.
// impact: An attacker can inject malicious SQL or NoSQL operators via the URL parameter, leading to data exfiltration.
// improvement: Validate and parameterize route params before using them in database queries.

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

interface Params {
  params: { slug: string };
}

export async function GET(request: NextRequest, { params }: Params) {
  const product = await prisma.$queryRawUnsafe(
    `SELECT * FROM products WHERE slug = '${params.slug}'`
  );
  return NextResponse.json(product);
}
