// [frensense]
// observation: res.sendFile() is called with 'hidden' and 'from' options. In Express 5.2.1, these options are renamed to 'dotfiles' and 'root'. The old option names are silently ignored.
// impact: The 'hidden: true' option is ignored, so dotfiles are not served even when intended. The 'from' option is ignored, so file resolution falls back to relative paths — potentially serving wrong files or failing with ENOENT.
// improvement: Use the Express 5.2.1 option names 'dotfiles' and 'root'.
// cwe: CWE-754
// cvss: 5.3
// owasp: 
// severity: Medium

import express, { Request, Response } from 'express';

const app = express();

app.get('/files/:file', (req: Request, res: Response) => {
  res.sendFile(req.params.file, { root: '/data/files', hidden: true });
});
