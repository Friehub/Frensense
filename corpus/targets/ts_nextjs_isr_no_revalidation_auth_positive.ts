// [frensense]
// observation: An ISR revalidation API endpoint (revalidatePath or on-demand revalidation) is exposed without authentication.
// impact: Any attacker who discovers the endpoint can trigger cache purges at will, causing performance degradation or bypassing cached content protections.
// improvement: Protect revalidation endpoints with a shared secret or authentication check.

import { NextRequest, NextResponse } from 'next/server';
import { revalidatePath } from 'next/cache';

export async function POST(request: NextRequest) {
  const { path } = await request.json();
  revalidatePath(path);
  return NextResponse.json({ revalidated: true });
}
