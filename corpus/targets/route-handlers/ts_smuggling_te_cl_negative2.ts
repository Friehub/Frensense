// SAFE alternative: configure a reverse proxy in front that normalizes transfer codings before they reach Express
import express from 'express';

const app = express();
app.use(express.json());

app.use((req, res, next) => {
  const rawTE = req.headers['transfer-encoding'] as string | undefined;
  if (rawTE && rawTE.toLowerCase() === 'chunked') {
    req.headers['content-length'] = Buffer.byteLength(JSON.stringify(req.body)).toString();
    delete req.headers['transfer-encoding'];
  }
  next();
});

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
