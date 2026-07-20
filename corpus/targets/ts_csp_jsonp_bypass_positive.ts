// [frensense]
// observation: The Content-Security-Policy allows 'self' for script-src, and the site hosts a JSONP endpoint that reflects a callback parameter. An attacker can bypass CSP by loading the JSONP endpoint as a script and exfiltrating data through the callback.
// impact: Even though CSP blocks external scripts, an attacker can use the site's own JSONP endpoint with a crafted callback to execute arbitrary JavaScript in the victim's browser, bypassing script-src: 'self'.
// improvement: Disable or remove JSONP endpoints, or add a separate nonce/hash requirement for script-src rather than relying on 'self' for pages that expose JSONP.

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

app.get('/api/jsonp', (req, res) => {
  const callback = req.query.callback as string;
  const data = { user: 'alice', email: 'alice@example.com' };
  res.setHeader('Content-Type', 'application/javascript');
  res.send(`${callback}(${JSON.stringify(data)});`);
});

export function getUserData(req: express.Request, res: express.Response): void {
  const cb = req.query.cb as string;
  const payload = { id: 1, name: 'Bob' };
  res.setHeader('Content-Type', 'application/javascript');
  res.send(`${cb}(${JSON.stringify(payload)});`);
}
