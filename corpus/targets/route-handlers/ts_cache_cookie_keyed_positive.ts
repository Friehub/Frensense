// [frensense]
// observation: The response varies by cookie value but the cache key does not include the Cookie header, causing one user's personalized content to be served to all other users.
// impact: An attacker can set a specific cookie value, request the page, and poison the cache so that subsequent users receive the attacker's personalized content, potentially leaking session-bound data.
// improvement: Include the Cookie header (or the relevant cookie subset) in the cache key, or mark the response with Cache-Control: private.
// cwe: CWE-614
// cvss: 5.4
// owasp: A02:2021
// severity: Medium

import express from 'express';

const app = express();

app.get('/dashboard', (req, res) => {
  const theme = req.cookies.theme || 'light';
  const html = `<html><body class="${theme}"><h1>Welcome back</h1></body></html>`;
  res.setHeader('Content-Type', 'text/html');
  res.setHeader('Cache-Control', 'public, max-age=3600');
  res.send(html);
});

export function serveUserPage(req: express.Request, res: express.Response): void {
  const lang = req.cookies.lang || 'en';
  const banner = `<div data-lang="${lang}">Special offer</div>`;
  res.setHeader('Cache-Control', 'public, max-age=86400');
  res.send(banner);
}
