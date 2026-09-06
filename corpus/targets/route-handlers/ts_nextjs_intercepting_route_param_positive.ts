// [frensense]
// observation: intercepted route `(..)photo/[id]` receives `params.id` from the wrong router context — the parent route's param instead of the intercepted route's param
// impact: wrong data is fetched and displayed to the user; if params are user-controllable it can leak other users' data
// improvement: validate that the intercepted params match the expected route segment

import { use } from 'react'

async function getPhoto(id: string) {
  const res = await fetch(`/api/photos/${id}`)
  return res.json()
}

export default function InterceptedPhoto({ params }: { params: { id: string } }) {
  const photo = use(getPhoto(params.id))
  return <img src={photo.url} alt={photo.title} />
}
