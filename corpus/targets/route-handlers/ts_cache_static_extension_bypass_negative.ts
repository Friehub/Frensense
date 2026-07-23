// SAFE: set Cache-Control: no-cache on dynamic .js endpoints
import express from 'express';

const app = express();

app.get('/api/config.js', (req, res) => {
  const config = {
    apiKey: req.query.key || 'default',
    endpoint: '/api/v1',
  };
  res.setHeader('Content-Type', 'application/javascript');
  res.setHeader('Cache-Control', 'no-cache, no-store, must-revalidate');
  res.send(`window.APP_CONFIG = ${JSON.stringify(config)};`);
});

export function generateUserConfig(req: express.Request, res: express.Response): void {
  const userId = req.query.uid as string;
  const config = { userId, featureFlags: ['beta'] };
  res.setHeader('Content-Type', 'application/javascript');
  res.setHeader('Cache-Control', 'no-cache, no-store, must-revalidate');
  res.send(`window.USER_CONFIG = ${JSON.stringify(config)};`);
}
