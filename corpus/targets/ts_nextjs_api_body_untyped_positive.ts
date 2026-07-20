// [frensense]
// observation: An API route parses the request body as JSON but uses it directly without any type validation or schema check.
// impact: Malformed or malicious payloads can cause unexpected behavior, mass assignment, or injection attacks.
// improvement: Validate the request body against a strict schema before using its values.

import { NextRequest, NextResponse } from 'next/server';
import prisma from '@/lib/prisma';

export async function POST(request: NextRequest) {
  const body = await request.json();
  const user = await prisma.user.create({
    data: {
      email: body.email,
      name: body.name,
      role: body.role,
    },
  });
  return NextResponse.json(user);
}
