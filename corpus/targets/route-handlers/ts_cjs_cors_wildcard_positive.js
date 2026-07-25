// [frensense]
// observation: CORS is configured with origin: '*' allowing any domain to make cross-origin requests, and credentials are enabled via withCredentials, making the wildcard permissive.
// impact: Any malicious website can make authenticated cross-origin requests to the API, leading to data exfiltration and CSRF-style attacks.
// improvement: Set origin to a specific allowlist of trusted domains, or disable credentials when using wildcard.
// cwe: CWE-942
// cvss: 8.8
// owasp: A05:2021
// severity: High

const cors = require('cors');
const express = require('express');

const app = express();
app.use(cors({ origin: '*', credentials: true }));

app.get('/api/user', function(req, res) {
  res.json({ name: 'Alice', email: 'alice@example.com' });
});
