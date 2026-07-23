// SAFE: uses a type-safe schema to validate protocol URL parameters

import { app } from 'electron'
import { z } from 'zod'

const DeepLinkSchema = z.object({
  action: z.enum(['open-settings', 'open-profile']),
  id: z.string().max(36).optional(),
})

app.setAsDefaultProtocolClient('myapp')

app.on('open-url', (event, rawUrl) => {
  event.preventDefault()
  const parsed = new URL(rawUrl)
  const result = DeepLinkSchema.safeParse({
    action: parsed.pathname.replace('/', ''),
    id: parsed.searchParams.get('id'),
  })
  if (!result.success) return
  const { action, id } = result.data
  if (action === 'open-profile' && id) loadProfile(id)
})

function loadProfile(id: string) {
  console.log(`Loading profile ${id}`)
}
