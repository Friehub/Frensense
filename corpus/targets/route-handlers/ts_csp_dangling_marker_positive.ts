// [frensense]
// observation: The CSP allows 'self' for img-src, and the page reflects user input inside a <script> tag. An attacker can inject a dangling markup payload that starts an unclosed string, causing the browser to interpret subsequent HTML as a string literal until a matching delimiter is found.
// impact: An attacker can inject <script>var x="<!-- and use the dangling marker to capture CSRF tokens or other sensitive data from the page context, exfiltrating them via an <img> tag.
// improvement: Encode all user input before reflection, use nonces for inline scripts, and avoid reflecting user input inside script blocks.

import helmet from 'helmet';
import express from 'express';

const app = express();

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", "'unsafe-inline'"],
  },
}));

app.get('/search', (req, res) => {
  const query = req.query.q as string;
  res.send(`
    <html>
      <script>
        var searchQuery = "${query}";
        document.getElementById('results').innerHTML = searchQuery;
      </script>
    </html>
  `);
});

export function showResult(req: express.Request, res: express.Response): void {
  const term = req.query.term as string;
  res.send(`<script>var term = "${term}";</script>`);
}
