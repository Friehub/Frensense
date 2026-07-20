// [frensense]
// observation: Remix action processes form data and persists it without validation
// impact: malformed data leads to stored XSS, data corruption, or injection attacks
// improvement: validate all input fields with zod or similar before database operation

import type { ActionFunctionArgs } from '@remix-run/node'
import { json, redirect } from '@remix-run/node'
import { db } from '~/db.server'

export async function action({ request }: ActionFunctionArgs) {
  const formData = await request.formData()
  const title = formData.get('title') as string
  const content = formData.get('content') as string

  const post = await db.post.create({
    data: { title, content }
  })

  return redirect(`/posts/${post.id}`)
}
