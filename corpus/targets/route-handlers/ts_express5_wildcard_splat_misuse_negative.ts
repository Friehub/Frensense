// SAFE: Use '/*splat' to match all paths including root in Express 5.2.1.

import express, { Request, Response } from 'express';

const app = express();

app.get('/*splat', (req: Request, res: Response) => {
  res.status(404).json({ error: 'Not found' });
});
