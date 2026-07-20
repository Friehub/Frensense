// SAFE: updateTag is called from a server-only context with the tag derived from a known internal event, not from user input

import { updateTag } from 'next/cache'

export async function POST(request: Request) {
  const authHeader = request.headers.get('authorization')
  if (authHeader !== `Bearer ${process.env.INTERNAL_WEBHOOK_SECRET ?? ''}`) {
    return new Response('unauthorized', { status: 403 })
  }

  const { event } = await request.json() as { event: string }

  if (event === 'post.published') {
    updateTag('posts')
  } else if (event === 'product.updated') {
    updateTag('products')
  }

  return new Response('ok')
}
