// SAFE: Use res.sendFile with only the root option (default dotfiles behavior).

import express, { Request, Response } from 'express';
import path from 'node:path';

const app = express();

app.get('/files/:file', (req: Request, res: Response) => {
  const filePath = path.resolve('/data/files', req.params.file);
  res.sendFile(filePath);
});
