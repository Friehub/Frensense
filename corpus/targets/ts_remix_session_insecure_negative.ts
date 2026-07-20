// SAFE: session configured with strong secret from env var

import { createCookieSessionStorage } from '@remix-run/node'

export const { getSession, commitSession, destroySession } = createCookieSessionStorage({
  cookie: {
    name: '__session',
    secrets: [process.env.SESSION_SECRET!],
    sameSite: 'lax',
    httpOnly: true,
    secure: true,
    maxAge: 60 * 60 * 24
  }
})
