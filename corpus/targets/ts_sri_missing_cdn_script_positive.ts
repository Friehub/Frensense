// [frensense]
// observation: A script from a CDN is loaded in a server-rendered page without an integrity attribute. If the CDN is compromised or serves different content depending on the user agent, the script may contain malicious code.
// impact: A compromised CDN can serve modified JavaScript that steals user data, performs unauthorized actions, or injects malware. Without SRI, the browser has no way to verify the script's authenticity.
// improvement: Add the integrity attribute with the correct base64-encoded hash of the expected file content for all externally loaded scripts.

import express from 'express';

const app = express();

app.get('/app', (req, res) => {
  res.send(`
    <html>
      <head>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/react/18.2.0/umd/react.production.min.js"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/react-dom/18.2.0/umd/react-dom.production.min.js"></script>
      </head>
      <body>
        <div id="root"></div>
      </body>
    </html>
  `);
});

export function renderAppShell(): string {
  return `
    <script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>
    <script src="/static/app.js"></script>
  `;
}
