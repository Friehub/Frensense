// SAFE: Use correct Express 5.2.1 argument order: res.redirect(status, url).

import express, { Request, Response } from 'express';

const app = express();

app.post('/login', (req: Request, res: Response) => {
  res.redirect(302, '/dashboard');
});
