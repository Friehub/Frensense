// [frensense]
// observation: intercepted route `(..)photo/[id]` bypasses the layout's auth check
// impact: unauthenticated users access protected content via modal interception
// improvement: duplicate auth check in the intercepted route or use a shared guard
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { use } from 'react'

async function getPhoto(id: string) {
  const res = await fetch(`/api/photos/${id}`)
  return res.json()
}

export default function PhotoPage({ params }: { params: { id: string } }) {
  const photo = use(getPhoto(params.id))
  return <img src={photo.url} alt={photo.title} />
}
