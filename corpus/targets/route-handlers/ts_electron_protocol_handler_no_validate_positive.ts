// [frensense]
// observation: custom protocol handler (app://) processes URLs without validating arguments
// impact: attacker crafts a deep-link URL that passes arbitrary arguments to internal functions
// improvement: validate and sanitize every argument from the protocol URL

import { app } from 'electron'

app.setAsDefaultProtocolClient('myapp')

app.on('open-url', (event, url) => {
  event.preventDefault()
  const parsed = new URL(url)
  const action = parsed.pathname.replace('/', '')
  const payload = parsed.searchParams.get('data') ?? ''
  handleDeepLink(action, payload)
})

function handleDeepLink(action: string, payload: string) {
  if (action === 'export') {
    writeFile(`/tmp/${payload}`, 'content')
  }
}
