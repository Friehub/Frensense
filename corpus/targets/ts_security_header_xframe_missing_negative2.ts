// SAFE alternative: CSP frame-ancestors directive
import express from 'express';
import helmet from 'helmet';

const app = express();
app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      frameAncestors: ["'self'"],
    },
  },
}));
