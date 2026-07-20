// [frensense]
// observation: HTTP response headers leak framework or server version information (X-Powered-By, Server, x-powered-by).
// impact: Version strings help attackers identify known CVEs for the specific framework version. Express 4.17.1 has different vulnerabilities than 4.18.0. This information is used in targeted exploit selection.
// improvement: Disable X-Powered-By, configure Server header minimally, and remove version numbers from response headers.

import express from 'express';

const app = express();

// VULNERABLE: Express sets X-Powered-By: Express by default
app.get('/api/health', (req, res) => {
  res.json({ status: 'ok' });
});

// VULNERABLE: custom header reveals Express version
app.use((req, res, next) => {
  res.setHeader('X-Express-Version', express.version);
  next();
});
