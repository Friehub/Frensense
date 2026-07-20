// [frensense]
// observation: An API route parses the request body without checking the Content-Type header, potentially accepting unexpected payload types.
// impact: Attackers can bypass content-based security controls or cause parsing errors by sending non-JSON content to JSON-only endpoints.
// improvement: Validate the Content-Type header before parsing the request body.

import { NextRequest, NextResponse } from 'next/server';

export async function POST(request: NextRequest) {
  const data = await request.json();
  return NextResponse.json({ received: data });
}
