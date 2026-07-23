// SAFE: Disable symlink following in express.static to prevent directory traversal via symlinks.

import express from 'express';
import path from 'path';

const app = express();
app.use('/static', express.static(path.join(__dirname, 'public'), { followSymlinks: false }));

app.get('/api/health', (req, res) => {
  res.json({ status: 'ok' });
});
