// [frensense]
// observation: The `updateTag()` API is called in a route handler or server action without any authentication or authorization check, allowing any client to invalidate server cache for arbitrary tags.
// impact: An attacker can continuously invalidate cache tags to degrade application performance or force re-computation of expensive operations, leading to denial of service via tag-flooding.
// improvement: Always validate the caller's identity and authorization before calling `updateTag()`, especially in route handlers.

import { NextRequest, NextResponse } from 'next/server'
import { updateTag } from 'next/cache'

export async function POST(request: NextRequest) {
  const { tag } = await request.json() as { tag: string }
  updateTag(tag)
  return NextResponse.json({ updated: true })
}
