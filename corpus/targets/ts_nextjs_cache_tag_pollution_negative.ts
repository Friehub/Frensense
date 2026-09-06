// SAFE: The tag name is validated against a known set of tags before being passed to revalidateTag

import { revalidateTag } from 'next/cache'
import { NextRequest, NextResponse } from 'next/server'

const ALLOWED_TAGS = new Set(['posts', 'products', 'users', 'settings'])

export async function POST(request: NextRequest) {
  const { tag } = await request.json() as { tag: string }
  if (!ALLOWED_TAGS.has(tag)) {
    return NextResponse.json({ error: 'invalid tag' }, { status: 400 })
  }
  revalidateTag(tag)
  return NextResponse.json({ revalidated: true })
}
