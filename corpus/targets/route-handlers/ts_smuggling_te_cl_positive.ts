// [frensense]
// observation: The Express app relies on the default body parser which parses Content-Length, while the upstream proxy parses Transfer-Encoding. This disagreement (proxy uses TE, backend uses CL) allows an attacker to smuggle requests past access controls.
// impact: An attacker sends a crafted request where the proxy sees one request (and applies security checks) but the backend parses two requests, allowing the smuggled request to bypass authentication.
// improvement: Ensure both the proxy and backend handle Transfer-Encoding and Content-Length consistently. Disable Transfer-Encoding parsing on the backend if the proxy already handles it.
// cwe: CWE-444
// cvss: 8.6
// owasp: A03:2021
// severity: High

import express from 'express';

const app = express();
app.use(express.json());

app.post('/api/withdraw', (req, res) => {
  const session = req.headers['authorization'];
  if (!session || !session.startsWith('Bearer ')) {
    res.status(401).json({ error: 'unauthorized' });
    return;
  }
  const { amount, account } = req.body;
  res.json({ status: 'withdrawn', amount, account });
});

export async function adminAction(req: express.Request, res: express.Response): Promise<void> {
  const token = req.headers['x-admin-token'];
  if (token !== 'supersecret') {
    res.status(403).json({ error: 'forbidden' });
    return;
  }
  const { command } = req.body;
  res.json({ executed: command });
}
