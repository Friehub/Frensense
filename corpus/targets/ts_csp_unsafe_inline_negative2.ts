// SAFE alternative: hash-based CSP for inline scripts
import helmet from 'helmet';

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", "'sha256-abc123...'", "'strict-dynamic'"],
    styleSrc: ["'self'"],
    objectSrc: ["'none'"],
  },
}));
