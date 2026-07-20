// SAFE: Revalidation endpoint validates a signed token embedded in the request body

import { NextRequest, NextResponse } from 'next/server';
import { revalidateTag } from 'next/cache';
import crypto from 'crypto';

function verifyToken(token: string): boolean {
  const expected = crypto
    .createHmac('sha256', process.env.REVALIDATION_SECRET!)
    .update('revalidate')
    .digest('hex');
  return crypto.timingSafeEqual(Buffer.from(token), Buffer.from(expected));
}

export async function POST(request: NextRequest) {
  const { tag, token } = await request.json();
  if (typeof token !== 'string' || !verifyToken(token)) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }
  if (typeof tag !== 'string' || tag.length === 0) {
    return NextResponse.json({ error: 'Invalid tag' }, { status: 400 });
  }
  revalidateTag(tag);
  return NextResponse.json({ revalidated: true });
}
