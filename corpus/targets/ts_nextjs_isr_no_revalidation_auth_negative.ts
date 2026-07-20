// SAFE: Revalidation endpoint requires a shared secret token in the request header

import { NextRequest, NextResponse } from 'next/server';
import { revalidatePath } from 'next/cache';

export async function POST(request: NextRequest) {
  const authHeader = request.headers.get('authorization');
  if (authHeader !== `Bearer ${process.env.REVALIDATION_SECRET}`) {
    return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
  }
  const { path } = await request.json();
  if (typeof path !== 'string' || !path.startsWith('/')) {
    return NextResponse.json({ error: 'Invalid path' }, { status: 400 });
  }
  revalidatePath(path);
  return NextResponse.json({ revalidated: true });
}
