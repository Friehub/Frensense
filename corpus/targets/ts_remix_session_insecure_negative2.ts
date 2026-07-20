// SAFE: session uses createSessionStorage with strong secret and proper flags

import { createSessionStorage } from '@remix-run/node'

const sessionStorage = createSessionStorage({
  cookie: {
    name: '__session',
    secrets: [process.env.SESSION_SECRET!],
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'strict',
    maxAge: 60 * 60 * 24 * 7
  },
  async createData(data) {
    return data.id
  },
  async readData(id) {
    return { id }
  },
  async updateData(id, data) {
    return data
  },
  async deleteData(id) {}
})

export const { getSession, commitSession, destroySession } = sessionStorage
