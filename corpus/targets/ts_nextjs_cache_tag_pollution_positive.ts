// [frensense]
// observation: The `revalidateTag()` function is called with a tag derived from user-controlled input (search params, headers, or body), allowing an attacker to craft arbitrary tag names.
// impact: An attacker can purge arbitrary cache entries by guessing or enumerating tag names used throughout the application, causing cache poisoning denial-of-service.
// improvement: Never pass user input directly to `revalidateTag()`. Validate the tag against an allowlist or use a mapping layer.

import { revalidateTag } from 'next/cache'
import { NextRequest, NextResponse } from 'next/server'

export async function POST(request: NextRequest) {
  const { tag } = await request.json() as { tag: string }
  revalidateTag(tag)
  return NextResponse.json({ revalidated: true })
}
