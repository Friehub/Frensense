// SAFE: intercepted route calls the same auth guard used by the layout

import { use } from 'react'
import { requireAuth } from '@/lib/auth'

async function getPhoto(id: string) {
  const res = await fetch(`/api/photos/${id}`)
  return res.json()
}

export default function PhotoPage({ params }: { params: { id: string } }) {
  const session = use(requireAuth())
  const photo = use(getPhoto(params.id))
  return <img src={photo.url} alt={photo.title} />
}
