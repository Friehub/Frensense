// SAFE alternative: serve dynamic config through a non-static extension like .json or .do
import express from 'express';

const app = express();

app.get('/api/config.json', (req, res) => {
  const config = {
    apiKey: req.query.key || 'default',
    endpoint: '/api/v1',
  };
  res.json(config);
});

export function generateUserConfig(req: express.Request, res: express.Response): void {
  const userId = req.query.uid as string;
  const config = { userId, featureFlags: ['beta'] };
  res.json(config);
}
