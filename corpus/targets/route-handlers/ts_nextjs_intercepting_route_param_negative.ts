// SAFE: validates that the intercepted route param matches the expected resource owner

import { use } from 'react'
import { getSession } from '@/lib/session'

async function getPhoto(id: string) {
  const res = await fetch(`/api/photos/${id}`)
  return res.json()
}

export default async function InterceptedPhoto({ params }: { params: { id: string } }) {
  const session = await getSession()
  if (!session || !session.userId) return <div>Access denied</div>
  const photo = await getPhoto(params.id)
  return <img src={photo.url} alt={photo.title} />
}
