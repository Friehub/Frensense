// SAFE: Use custom middleware with realpath checks to block symlinks.

import express, { Request, Response, NextFunction } from 'express';
import path from 'node:path';
import fs from 'node:fs';
import process from 'node:process';

const app = express();

const publicDir = path.join(process.cwd(), 'public');

app.use('/static', (req: Request, res: Response, next: NextFunction) => {
  const filePath = path.join(publicDir, req.path);
  const real = fs.realpathSync(filePath);
  if (!real.startsWith(publicDir)) {
    res.status(403).json({ error: 'Forbidden' });
    return;
  }
  next();
}, express.static(publicDir));
