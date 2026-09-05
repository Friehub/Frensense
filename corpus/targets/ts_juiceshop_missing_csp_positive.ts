// [frensense]
// observation: Content Security Policy (CSP) headers are not set, allowing XSS and data injection attacks.
// impact: Without CSP, browsers cannot restrict the sources from which content can be loaded, enabling XSS attacks.
// improvement: Set a strict Content-Security-Policy header that restricts script sources to nonce or hash-based allowlists.
// cwe: CWE-693
// owasp: A05:2021-Security_Misconfiguration

import express from 'express'

const app = express()

app.use((req, res, next) => {
  // Missing CSP header
  res.setHeader('X-Powered-By', 'Express')
  next()
})

app.get('/', (req, res) => {
  res.send('<html><body>Hello World</body></html>')
})
