// [frensense]
// observation: express.static() is used without setting symlinks: false. Express 5.2.1 follows symlinks by default when serving static files
// impact: An attacker who uploads a symlink in the public directory can serve arbitrary filesystem paths like /etc/passwd or /proc/self/environ, bypassing the intended root directory restriction
// improvement: Set symlinks: false in express.static() options to prevent following symlinks

import express from 'express';
import path from 'node:path';

function serveStaticFiles(app: express.Application): void {
  app.use('/static', express.static(path.join(process.cwd(), 'public')));
}
