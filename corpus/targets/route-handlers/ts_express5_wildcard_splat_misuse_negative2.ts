// SAFE: Use '{*splat}' syntax for explicit wildcard capture in Express 5.2.1.

import express, { Request, Response } from 'express';

const app = express();

app.get('/{*splat}', (req: Request, res: Response) => {
  const path = req.params.splat;
  res.status(404).json({ error: 'Not found', path });
});
