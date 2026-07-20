// SAFE: The rewrite destination is validated against an allowlist of permitted targets

import { NextRequest, NextResponse } from 'next/server';

const ALLOWED_TARGETS = ['/dashboard', '/profile', '/settings', '/help'];

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const dest = searchParams.get('url') || '/';
  if (!ALLOWED_TARGETS.includes(dest) && !dest.startsWith('/api/')) {
    return NextResponse.json({ error: 'Invalid redirect target' }, { status: 400 });
  }
  return NextResponse.redirect(new URL(dest, request.url));
}
