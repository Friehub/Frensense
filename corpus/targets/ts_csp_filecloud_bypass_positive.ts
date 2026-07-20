// [frensense]
// observation: The Content-Security-Policy includes a CDN domain that allows user-uploaded files. If the CDN serves user-controlled content with a JavaScript MIME type, an attacker can upload a malicious script and have it execute on the victim's page.
// impact: An attacker uploads a .js file to the user-content CDN domain, and since the CSP allows that domain in script-src, the browser loads and executes the attacker's script, bypassing the CSP.
// improvement: Do not include user-content CDN domains in script-src. If necessary, use a dedicated subdomain that only serves static, user-uploaded content with Content-Disposition: attachment.

import helmet from 'helmet';
import express from 'express';

const app = express();

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", 'https://storage.example.com'],
    imgSrc: ["'self'", 'https://storage.example.com'],
  },
}));

export function renderProfilePage(req: express.Request, res: express.Response): void {
  res.send(`
    <html>
      <head>
        <script src="https://storage.example.com/uploads/widget.js"></script>
      </head>
      <body>Profile</body>
    </html>
  `);
}
