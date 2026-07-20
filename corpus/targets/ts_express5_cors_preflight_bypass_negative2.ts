// SAFE: Use the cors npm package which handles preflight automatically.

import express, { Request, Response } from 'express';
import cors from 'cors';

const app = express();
app.use(cors({ origin: 'https://trusted.example.com' }));

app.post('/api/data', (req: Request, res: Response) => {
  res.json({ success: true });
});
