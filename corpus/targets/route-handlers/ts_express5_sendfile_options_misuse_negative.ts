// SAFE: Use correct Express 5.2.1 option names: dotfiles and root.

import express, { Request, Response } from 'express';

const app = express();

app.get('/files/:file', (req: Request, res: Response) => {
  res.sendFile(req.params.file, { root: '/data/files', dotfiles: 'allow' });
});
