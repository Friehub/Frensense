// SAFE alternative: replace JSONP with a POST-only endpoint that returns JSON (no reflection)
import helmet from 'helmet';
import express from 'express';

const app = express();

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'"],
    objectSrc: ["'none'"],
  },
}));

app.post('/api/user', (req, res) => {
  const data = { user: 'alice', email: 'alice@example.com' };
  res.json(data);
});

export function fetchUserData(req: express.Request, res: express.Response): void {
  const payload = { id: 1, name: 'Bob' };
  res.json(payload);
}
