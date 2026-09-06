// SAFE: uses server-side fetch with ownership check inside the intercepted route

import { getPhoto } from '@/lib/photos'
import { requireOwnership } from '@/lib/auth'

export default async function InterceptedPhoto({ params }: { params: { id: string } }) {
  await requireOwnership(params.id)
  const photo = await getPhoto(params.id)
  return <img src={photo.url} alt={photo.title} />
}
