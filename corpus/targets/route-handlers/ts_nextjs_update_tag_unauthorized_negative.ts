// SAFE: updateTag is protected by an authentication check before execution

import { NextRequest, NextResponse } from 'next/server'
import { updateTag } from 'next/cache'

async function getSession(request: NextRequest) {
  const token = request.cookies.get('session-token')?.value
  if (!token) return null
  const res = await fetch('https://auth.internal/session', {
    headers: { Authorization: `Bearer ${token}` },
  })
  if (!res.ok) return null
  return res.json() as Promise<{ userId: string; role: string }>
}

export async function POST(request: NextRequest) {
  const session = await getSession(request)
  if (!session || session.role !== 'admin') {
    return NextResponse.json({ error: 'unauthorized' }, { status: 403 })
  }

  const { tag } = await request.json() as { tag: string }
  updateTag(tag)
  return NextResponse.json({ updated: true })
}
