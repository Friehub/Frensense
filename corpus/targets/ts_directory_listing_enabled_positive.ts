// [frensense]
// observation: Express static middleware serves entire directories without the dotfiles option or a deny file, enabling clients to browse the directory listing
// impact: Attackers can enumerate all files in the served directories, discovering backup archives, internal configs, node_modules traversal, or accidentally deployed hidden files
// improvement: Disable directory listing by using serve-static with { dotfiles: 'deny' } and never serve the parent directory; use specific file-serving routes instead

import express from 'express';
import { join } from 'path';

function setupStaticFiles(app: express.Application): void {
  app.use('/files', express.static(join(__dirname, 'public')));
}
