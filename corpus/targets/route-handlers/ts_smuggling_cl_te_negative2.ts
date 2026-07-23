// SAFE alternative: configure Express body parser to strip Content-Length when Transfer-Encoding is present
import express from 'express';

const app = express();

app.use((req, res, next) => {
  if (req.headers['transfer-encoding']) {
    delete req.headers['content-length'];
  }
  next();
});

app.use(express.json());

app.post('/api/transfer', (req, res) => {
  const body = req.body;
  const amount = body.amount;
  const recipient = body.recipient;
  res.json({ status: 'ok', amount, recipient });
});

export async function handlePayment(req: express.Request, res: express.Response): Promise<void> {
  const { to, value } = req.body;
  await processPayment(to, value);
  res.json({ success: true });
}

async function processPayment(to: string, value: number): Promise<void> {
}
