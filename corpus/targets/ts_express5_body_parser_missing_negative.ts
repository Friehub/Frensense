// SAFE: Register express.json() body parser before route handlers.

import express, { Request, Response } from 'express';

const app = express();
app.use(express.json());

app.post('/api/login', (req: Request, res: Response) => {
  const email = req.body.email;
  res.json({ email });
});
