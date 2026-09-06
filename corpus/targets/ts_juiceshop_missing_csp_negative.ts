// SAFE: Sets Content Security Policy header
import express from 'express'

const app = express()

app.use((req, res, next) => {
  res.setHeader(
    'Content-Security-Policy',
    "default-src 'self'; script-src 'self' 'nonce-abc123'; style-src 'self' 'unsafe-inline'"
  )
  res.setHeader('X-Powered-By', 'Express')
  next()
})

app.get('/', (req, res) => {
  res.send('<html><body>Hello World</body></html>')
})
