// SAFE alternative: use a strict CSP with nonces and keep CDN only for images
import helmet from 'helmet';
import crypto from 'node:crypto';

const app = express();

app.use((req, res, next) => {
  res.locals.nonce = crypto.randomBytes(16).toString('hex');
  next();
});

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", (req, res) => `'nonce-${res.locals.nonce}'`],
    imgSrc: ["'self'", 'https://storage.example.com'],
  },
}));

export function renderProfilePage(req: express.Request, res: express.Response): void {
  const nonce = res.locals.nonce;
  res.send(`
    <html>
      <head>
        <script nonce="${nonce}" src="/static/widget.js"></script>
      </head>
      <body>Profile</body>
    </html>
  `);
}
