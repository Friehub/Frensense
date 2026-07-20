// [frensense]
// observation: URL searchParams are used directly in a database query without sanitization, enabling NoSQL or SQL injection.
// impact: An attacker can craft search parameters to inject query operators ($gt, $regex, OR 1=1) and exfiltrate or corrupt data.
// improvement: Validate and coerce searchParams to expected types before using them in database queries.

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const users = await prisma.user.findMany({
    where: {
      email: { contains: searchParams.get('q') as string },
    },
  });
  return NextResponse.json(users);
}
