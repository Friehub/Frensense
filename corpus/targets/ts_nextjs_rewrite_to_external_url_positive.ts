// [frensense]
// observation: Next.js rewrites configuration uses a user-controlled destination URL, enabling open redirect or SSRF via rewritten paths.
// impact: An attacker can redirect traffic to malicious external sites or internal services by controlling the rewrite destination.
// improvement: Ensure rewrite destinations are hardcoded or validated against an allowlist of trusted URLs.

import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const dest = searchParams.get('url') || '/';
  return NextResponse.redirect(new URL(dest, request.url));
}
