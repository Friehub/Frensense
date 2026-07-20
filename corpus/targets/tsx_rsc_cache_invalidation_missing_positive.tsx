// [frensense]
// observation: fetch result is cached by Next.js but never revalidated after the mutation — subsequent reads return stale data
// impact: users see stale content after data mutations; if this involves auth state, privileges or billing, it's a security bypass
// improvement: call `revalidatePath()` or `revalidateTag()` after mutations

import { revalidatePath } from 'next/cache'

export default async function PostList() {
  const posts = await fetch('https://api.example.com/posts', { next: { tags: ['posts'] } }).then(r => r.json())
  return (
    <ul>
      {posts.map((p: { id: number; title: string }) => (
        <li key={p.id}>{p.title}</li>
      ))}
    </ul>
  )
}

export async function createPost(data: FormData) {
  'use server'
  await fetch('https://api.example.com/posts', { method: 'POST', body: data })
}
