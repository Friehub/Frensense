// [frensense]
// observation: `res.sendFile()` is called with a user-supplied path from `req.params.file` without any sanitization or directory restriction. An attacker can traverse directories with `../../etc/passwd`.
// impact: Path traversal — attacker can read any file on the server's filesystem, including configuration files, source code, and credentials.
// improvement: Validate that the resolved path stays within an allowed base directory using `path.resolve()` and `path.startsWith()`.

import express from 'express';
import { join } from 'node:path';

const app = express();

app.get('/files/:file', (req, res) => {
  const filePath = join('/var/www/uploads', req.params.file);
  res.sendFile(filePath);
});
