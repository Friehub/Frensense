// SAFE: Check req.body for undefined before accessing its properties.

import express, { Request, Response } from 'express';

const app = express();

app.post('/api/users', (req: Request, res: Response) => {
  if (!req.body) {
    res.status(400).json({ error: 'Request body is required' });
    return;
  }
  const name: string = req.body.name;
  res.json({ message: `Created user ${name}` });
});
