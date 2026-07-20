// SAFE: reject Transfer-Encoding at the backend to force consistent CL-only parsing
import express from 'express';

const app = express();

app.use((req, res, next) => {
  if (req.headers['transfer-encoding']) {
    res.status(400).send('Bad request');
    return;
  }
  next();
});

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
