// [frensense]
// observation: The Express app uses the X-Forwarded-Host header to construct a redirect URL but does not include it in the cache key. The CDN caches the response keyed only by the request path.
// impact: An attacker can send a request with a malicious X-Forwarded-Host header, poisoning the cache so that subsequent users are redirected to an attacker-controlled domain.
// improvement: Include all headers used to generate responses in the cache key, or validate and normalize the X-Forwarded-Host header by whitelisting allowed hosts.

import express from 'express';

const app = express();

app.get('/redirect', (req, res) => {
  const host = req.headers['x-forwarded-host'] as string || req.headers.host as string;
  const redirectUrl = `https://${host}/callback?code=abc`;
  res.redirect(302, redirectUrl);
});

export function setupRedirectRoutes(server: express.Application): void {
  server.get('/auth/complete', (req, res) => {
    const target = req.headers['x-forwarded-host'] as string;
    res.redirect(`https://${target}/welcome`);
  });
}
