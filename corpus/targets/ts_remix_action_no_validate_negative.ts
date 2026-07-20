// SAFE: input validated with zod schema before database write

import type { ActionFunctionArgs } from '@remix-run/node'
import { json, redirect } from '@remix-run/node'
import { db } from '~/db.server'
import { z } from 'zod'

const postSchema = z.object({
  title: z.string().min(1).max(200),
  content: z.string().min(1).max(10000)
})

export async function action({ request }: ActionFunctionArgs) {
  const formData = await request.formData()
  const raw = {
    title: formData.get('title') as string,
    content: formData.get('content') as string
  }

  const parsed = postSchema.safeParse(raw)
  if (!parsed.success) {
    return json({ errors: parsed.error.flatten().fieldErrors }, { status: 400 })
  }

  const post = await db.post.create({
    data: parsed.data
  })

  return redirect(`/posts/${post.id}`)
}
