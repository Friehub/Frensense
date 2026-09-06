// [frensense]
// observation: The server routes a dynamic API endpoint with a .js extension, which the CDN treats as a static asset and caches aggressively, serving stale or poisoned responses.
// impact: An attacker-controlled dynamic response (e.g., error message with reflected input) gets cached as a static .js file, serving malicious content to all users who load that script.
// improvement: Use non-cacheable extensions for dynamic routes, or add explicit Cache-Control: no-cache headers to dynamic endpoints regardless of URL pattern.

import express from 'express';

const app = express();

app.get('/api/config.js', (req, res) => {
  const config = {
    apiKey: req.query.key || 'default',
    endpoint: '/api/v1',
  };
  res.setHeader('Content-Type', 'application/javascript');
  res.setHeader('Cache-Control', 'public, max-age=31536000');
  res.send(`window.APP_CONFIG = ${JSON.stringify(config)};`);
});

export function generateUserConfig(req: express.Request, res: express.Response): void {
  const userId = req.query.uid as string;
  const config = { userId, featureFlags: ['beta'] };
  res.setHeader('Content-Type', 'application/javascript');
  res.setHeader('Cache-Control', 'public, max-age=86400');
  res.send(`window.USER_CONFIG = ${JSON.stringify(config)};`);
}
