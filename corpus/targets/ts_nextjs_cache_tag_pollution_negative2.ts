// SAFE: User input is mapped to a fixed tag via a lookup table, preventing arbitrary tag injection

import { revalidateTag } from 'next/cache'
import { NextRequest, NextResponse } from 'next/server'

const TAG_MAP: Record<string, string> = {
  blog: 'posts',
  shop: 'products',
  profile: 'users',
}

export async function POST(request: NextRequest) {
  const { section } = await request.json() as { section: string }
  const tag = TAG_MAP[section]
  if (!tag) {
    return NextResponse.json({ error: 'unknown section' }, { status: 400 })
  }
  revalidateTag(tag)
  return NextResponse.json({ revalidated: true })
}
