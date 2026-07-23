// SAFE: Use a custom handler to serve .well-known files explicitly.

import express, { Request, Response } from 'express';
import path from 'node:path';
import fs from 'node:fs';

const app = express();

app.use('/.well-known', (req: Request, res: Response) => {
  const filePath = path.join('public', req.path);
  if (fs.existsSync(filePath)) {
    res.sendFile(path.resolve(filePath));
  } else {
    res.status(404).end();
  }
});

app.use(express.static('public'));
