// [frensense]
// observation: express.static() serves the public directory without explicit dotfiles options. Express 5.2.1 defaults to dotfiles: 'ignore', so .well-known ACME challenge files are silently ignored
// impact: ACME HTTP-01 challenges at /.well-known/acme-challenge/ return 404, causing Let's Encrypt certificate renewal to fail and HTTPS certificates to expire
// improvement: Set dotfiles: 'allow' for the specific static mount, or serve .well-known with a custom handler

import express from 'express';

function setupStaticFiles(app: express.Application): void {
  app.use(express.static('public'));
}
