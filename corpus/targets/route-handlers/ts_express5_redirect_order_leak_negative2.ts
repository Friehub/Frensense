// SAFE: Use status() then redirect() for clarity.

import express, { Request, Response } from 'express';

const app = express();

app.post('/login', (req: Request, res: Response) => {
  res.status(302).redirect('/dashboard');
});
