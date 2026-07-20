// SAFE: validates action against an allowlist and sanitizes the payload

import { app } from 'electron'

const ALLOWED_ACTIONS = new Set(['open-settings', 'open-profile'])

function sanitizePayload(raw: string): string {
  return raw.replace(/[^a-zA-Z0-9_-]/g, '')
}

app.setAsDefaultProtocolClient('myapp')

app.on('open-url', (event, url) => {
  event.preventDefault()
  const parsed = new URL(url)
  const action = parsed.pathname.replace('/', '')
  if (!ALLOWED_ACTIONS.has(action)) return
  const payload = sanitizePayload(parsed.searchParams.get('data') ?? '')
  handleDeepLink(action, payload)
})

function handleDeepLink(action: string, payload: string) {
  if (action === 'open-profile') {
    loadProfile(payload)
  }
}
