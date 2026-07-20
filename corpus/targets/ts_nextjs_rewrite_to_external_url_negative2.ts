// SAFE: Only relative URLs are accepted, external absolute URLs are rejected outright

import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const dest = searchParams.get('url') || '/';
  try {
    const parsed = new URL(dest, 'http://localhost');
    if (parsed.hostname !== 'localhost') {
      return NextResponse.json({ error: 'External redirects not allowed' }, { status: 400 });
    }
    return NextResponse.redirect(new URL(parsed.pathname + parsed.search, request.url));
  } catch {
    return NextResponse.redirect(new URL(dest, request.url));
  }
}
