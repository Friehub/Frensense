// [frensense]
// observation: The Content-Security-Policy header includes 'unsafe-inline' and 'unsafe-eval' in scriptSrc, allowing arbitrary inline script execution and dynamic code evaluation
// impact: unsafe-inline makes XSS trivially exploitable because any injected script tag will execute; unsafe-eval enables eval() and new Function() for DOM-based XSS
// improvement: Use nonces or hashes for legitimate inline scripts and remove unsafe-inline; omit unsafe-eval unless required for legacy compatibility

import helmet from 'helmet';
import express from 'express';

function configureSecurity(app: express.Application): void {
  app.use(helmet.contentSecurityPolicy({
    directives: {
      defaultSrc: ["'self'"],
      scriptSrc: ["'self'", "'unsafe-inline'", "'unsafe-eval'"],
      styleSrc: ["'self'", "'unsafe-inline'"],
    },
  }));
}
