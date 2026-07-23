// SAFE: Register a body parser so req.body is always defined, then use defaults.

import express, { Request, Response } from 'express';

const app = express();
app.use(express.json());

app.post('/api/users', (req: Request, res: Response) => {
  const name: string = req.body?.name ?? '';
  res.json({ message: `Created user ${name}` });
});
