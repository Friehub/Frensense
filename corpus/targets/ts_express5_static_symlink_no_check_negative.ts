// SAFE: Disable symlink following in express.static() options.

import express from 'express';
import path from 'node:path';

const app = express();

app.use('/static', express.static(path.join(process.cwd(), 'public'), { symlinks: false }));
