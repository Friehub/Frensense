// SAFE: reject requests with both Content-Length and Transfer-Encoding headers
import express from 'express';

const app = express();

function rejectSmuggled(req: express.Request, res: express.Response, next: express.NextFunction): void {
  const hasCL = req.headers['content-length'] !== undefined;
  const hasTE = req.headers['transfer-encoding'] !== undefined;
  if (hasCL && hasTE) {
    res.status(400).send('Bad request');
    return;
  }
  next();
}

app.use(rejectSmuggled);

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
