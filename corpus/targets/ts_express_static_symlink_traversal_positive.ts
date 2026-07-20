// [frensense]
// observation: express.static is used with default options, which follows symlinks. A user who can create a symlink inside the served directory can escape and serve arbitrary files.
// impact: An attacker can read sensitive files outside the static directory (e.g., /etc/passwd, .env, config files) by creating a symlink pointing outside the root.
// improvement: Set followSymlinks: false in the express.static options to prevent symlink following.

import express from 'express';
import path from 'path';

const app = express();
app.use('/static', express.static(path.join(__dirname, 'public')));

app.get('/api/health', (req, res) => {
  res.json({ status: 'ok' });
});
