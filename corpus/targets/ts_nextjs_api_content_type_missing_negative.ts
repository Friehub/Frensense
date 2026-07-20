// SAFE: Content-Type is validated before parsing the body

import { NextRequest, NextResponse } from 'next/server';

export async function POST(request: NextRequest) {
  const contentType = request.headers.get('content-type') || '';
  if (!contentType.includes('application/json')) {
    return NextResponse.json({ error: 'Content-Type must be application/json' }, { status: 415 });
  }
  const data = await request.json();
  return NextResponse.json({ received: data });
}
