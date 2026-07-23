// SAFE: Register both JSON and URL-encoded body parsers for form and API data.

import express, { Request, Response } from 'express';

const app = express();
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

app.post('/api/login', (req: Request, res: Response) => {
  const email = req.body.email;
  res.json({ email });
});
