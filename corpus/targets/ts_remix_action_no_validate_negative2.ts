// SAFE: manual validation with HTML escaping before persistence

import type { ActionFunctionArgs } from '@remix-run/node'
import { json, redirect } from '@remix-run/node'
import { db } from '~/db.server'

function escapeHtml(s: string): string {
  return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
}

export async function action({ request }: ActionFunctionArgs) {
  const formData = await request.formData()
  const title = (formData.get('title') as string || '').trim()
  const content = (formData.get('content') as string || '').trim()

  if (title.length === 0 || title.length > 200) {
    return json({ error: 'Title must be 1-200 characters' }, { status: 400 })
  }

  if (content.length === 0 || content.length > 10000) {
    return json({ error: 'Content must be 1-10000 characters' }, { status: 400 })
  }

  const post = await db.post.create({
    data: { title: escapeHtml(title), content: escapeHtml(content) }
  })

  return redirect(`/posts/${post.id}`)
}
