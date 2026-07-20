// SAFE: Route uses a strict Content-Type check with a helper that rejects non-JSON requests early

import { NextRequest, NextResponse } from 'next/server';

function requireJson(request: NextRequest): void {
  const ct = request.headers.get('content-type') || '';
  if (!ct.includes('application/json')) {
    throw new Error('Expected application/json');
  }
}

export async function POST(request: NextRequest) {
  try {
    requireJson(request);
    const data = await request.json();
    return NextResponse.json({ received: data });
  } catch (e) {
    return NextResponse.json(
      { error: e instanceof Error ? e.message : 'Bad request' },
      { status: 415 }
    );
  }
}
