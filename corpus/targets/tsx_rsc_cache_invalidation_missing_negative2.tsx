// SAFE: revalidates the path after mutation

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
  revalidatePath('/posts')
}
