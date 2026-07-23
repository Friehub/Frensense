// SAFE: validate the callback parameter to only allow safe identifiers
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

function safeCallbackName(name: string): boolean {
  return /^[a-zA-Z_$][0-9a-zA-Z_$]*$/.test(name);
}

app.get('/api/jsonp', (req, res) => {
  const callback = req.query.callback as string;
  if (!callback || !safeCallbackName(callback)) {
    res.status(400).json({ error: 'invalid callback' });
    return;
  }
  const data = { user: 'alice', email: 'alice@example.com' };
  res.setHeader('Content-Type', 'application/javascript');
  res.send(`${callback}(${JSON.stringify(data)});`);
});

export function getUserData(req: express.Request, res: express.Response): void {
  const cb = req.query.cb as string;
  if (!cb || !safeCallbackName(cb)) {
    res.status(400).json({ error: 'invalid callback' });
    return;
  }
  const payload = { id: 1, name: 'Bob' };
  res.setHeader('Content-Type', 'application/javascript');
  res.send(`${cb}(${JSON.stringify(payload)});`);
}
