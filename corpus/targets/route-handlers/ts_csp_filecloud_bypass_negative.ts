// SAFE: remove user-content CDN from script-src; serve scripts only from self
import helmet from 'helmet';
import express from 'express';

const app = express();

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'"],
    imgSrc: ["'self'", 'https://storage.example.com'],
  },
}));

export function renderProfilePage(req: express.Request, res: express.Response): void {
  res.send(`
    <html>
      <head>
        <script src="/static/widget.abc123.js"></script>
      </head>
      <body>Profile</body>
    </html>
  `);
}
